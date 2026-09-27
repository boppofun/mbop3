//! Frame level decoding and the bit reservoir.

use crate::bits::BitReader;
use crate::header::{HDR_SIZE, Header, find_frame};
use crate::layer3::{self, GrInfo};
use crate::synth::{HIST_ROWS, Hist, synth_granule};
use crate::{FrameInfo, MAX_SAMPLES_PER_FRAME, Sample};

const MAX_BITRESERVOIR_BYTES: usize = 511;

/// MP3 decoder state.
///
/// This holds all of the decoder's memory (including per-frame working
/// memory) so decoding uses little stack. Keep it on the heap
/// (`Decoder::new_boxed()`, feature `alloc`) or in a static: `Decoder::new()`
/// is all zeros so a static decoder is placed in `.bss`.
#[derive(Clone)]
#[cfg_attr(feature = "alloc", derive(zerocopy::FromZeros))]
pub struct Decoder {
    mdct_overlap: [[f32; 288]; 2],
    synth_hist: Hist,
    reserv: i32,
    free_format_bytes: i32,
    header: [u8; 4],
    reserv_buf: [u8; MAX_BITRESERVOIR_BYTES],
    /// Per channel: bands of mdct_overlap that may be nonzero.
    overlap_bands: [u8; 2],
    work: Work,
}

/// Working memory for one frame. Nothing here is carried between frames.
#[derive(Clone)]
#[cfg_attr(feature = "alloc", derive(zerocopy::FromZeros))]
struct Work {
    gr_info: [GrInfo; 4],
    grbuf: [f32; 576 * 2],
    scf: [f32; 40],
    ist_pos: [[u8; 39]; 2],
    /// Whether grbuf's channel 1 half may be nonzero (it is only used by
    /// stereo frames, and by rare corrupt mono ones).
    grbuf1_dirty: bool,
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder {
    pub const fn new() -> Self {
        Decoder {
            mdct_overlap: [[0.0; 288]; 2],
            synth_hist: [0.0; HIST_ROWS * 64],
            reserv: 0,
            free_format_bytes: 0,
            header: [0; 4],
            reserv_buf: [0; MAX_BITRESERVOIR_BYTES],
            overlap_bands: [0; 2],
            work: Work {
                gr_info: [GrInfo::ZERO; 4],
                grbuf: [0.0; 576 * 2],
                scf: [0.0; 40],
                ist_pos: [[0; 39]; 2],
                grbuf1_dirty: false,
            },
        }
    }

    /// A new decoder on the heap. Unlike `Box::new(Decoder::new())`, this never
    /// builds the (12 KB) decoder on the stack first.
    #[cfg(feature = "alloc")]
    pub fn new_boxed() -> alloc::boxed::Box<Self> {
        use zerocopy::FromZeros;
        // All zeros is exactly Decoder::new().
        Self::new_box_zeroed().unwrap_or_else(|_| {
            alloc::alloc::handle_alloc_error(core::alloc::Layout::new::<Self>())
        })
    }

    /// Resets the stream state, as when starting a new stream.
    fn reset(&mut self) {
        self.mdct_overlap = [[0.0; 288]; 2];
        self.synth_hist = [0.0; HIST_ROWS * 64];
        self.reserv = 0;
        self.free_format_bytes = 0;
        self.header = [0; 4];
        self.reserv_buf = [0; MAX_BITRESERVOIR_BYTES];
        self.overlap_bands = [0; 2];
    }

    /// Finds and decodes the next frame in `mp3`.
    ///
    /// Returns the number of samples per channel written to `pcm` and info
    /// about the frame. `info.frame_bytes` is how much input to skip: 0 means
    /// no frame was found and more input is needed. Some frames produce no
    /// samples (e.g. when the bit reservoir they refer to is missing).
    ///
    /// For reliable sync `mp3` should contain several frames (16 KiB or all
    /// the remaining input). With `pcm` None the frame is only parsed.
    pub fn decode_frame<S: Sample>(
        &mut self,
        mp3: &[u8],
        pcm: Option<&mut [S; MAX_SAMPLES_PER_FRAME]>,
    ) -> (usize, FrameInfo) {
        let mp3 = &mp3[..mp3.len().min(i32::MAX as usize)];
        let mp3_bytes = mp3.len() as i32;
        let mut info = FrameInfo::default();
        let mut i: i32 = 0;
        let mut frame_size: i32 = 0;

        if mp3_bytes > 4 && self.header[0] == 0xff && Header(self.header).compare(Header::at(mp3)) {
            let h = Header::at(mp3);
            frame_size = h.frame_bytes(self.free_format_bytes) + h.padding();
            if frame_size != mp3_bytes
                && (frame_size + HDR_SIZE as i32 > mp3_bytes
                    || !h.compare(Header::at(&mp3[frame_size as usize..])))
            {
                frame_size = 0;
            }
        }
        if frame_size == 0 {
            self.reset();
            (i, frame_size) = find_frame(mp3, &mut self.free_format_bytes);
            if frame_size == 0 || i + frame_size > mp3_bytes {
                info.frame_bytes = i;
                return (0, info);
            }
        }

        let frame = &mp3[i as usize..(i + frame_size) as usize];
        // A (corrupt) free format frame can be shorter than its header.
        let hdr = Header::at(&mp3[i as usize..]);
        self.header = hdr.0;
        info.frame_bytes = i + frame_size;
        info.frame_offset = i;
        info.channels = if hdr.is_mono() { 1 } else { 2 };
        info.hz = hdr.sample_rate_hz() as i32;
        info.layer = 4 - hdr.layer() as i32;
        info.bitrate_kbps = hdr.bitrate_kbps() as i32;

        let Some(pcm) = pcm else {
            return (hdr.frame_samples() as usize, info);
        };

        let payload = frame.get(HDR_SIZE..).unwrap_or(&[]);
        let mut bs_frame = BitReader::new(payload, frame_size - HDR_SIZE as i32);
        if hdr.is_crc() {
            bs_frame.get_bits(16);
        }

        if info.layer != 3 {
            // Layer 1 and 2 are not supported.
            return (0, info);
        }

        let w = &mut self.work;
        let main_data_begin = crate::timed!(
            0,
            layer3::read_side_info(&mut bs_frame, &mut w.gr_info, hdr)
        );
        if main_data_begin < 0 || bs_frame.pos > bs_frame.limit {
            self.header[0] = 0;
            return (0, info);
        }
        // Scalefactor reuse (scfsi) can read these before they are written.
        w.ist_pos = [[0; 39]; 2];

        // Main data: the end of the bit reservoir, then this frame's payload
        // after the side info.
        let frame_start = (bs_frame.pos / 8) as usize;
        let frame_main_len = ((bs_frame.limit - bs_frame.pos) / 8) as usize;
        let frame_main = &payload[frame_start..frame_start + frame_main_len];
        let bytes_have = self.reserv.min(main_data_begin) as usize;
        let reserv_from = (self.reserv - main_data_begin).max(0) as usize;
        let success = self.reserv >= main_data_begin;

        let (save_pos, save_len) = {
            let head = &self.reserv_buf[reserv_from..reserv_from + bytes_have];
            let mut bs = BitReader::new_split(head, frame_main);
            if success {
                let nch = info.channels as usize;
                let granules = if hdr.test_mpeg1() { 2 } else { 1 };
                for igr in 0..granules {
                    // minimp3 zeroes both channels; channel 1 is only zeroed
                    // when it may have been written.
                    let (ch0, ch1) = w.grbuf.split_at_mut(576);
                    ch0.fill(0.0);
                    if w.grbuf1_dirty || nch == 2 {
                        ch1.fill(0.0);
                    }
                    let gr = &w.gr_info[igr * nch..];
                    w.grbuf1_dirty = nch == 2 || hdr.test_i_stereo() || gr[0].n_short_sfb != 0;
                    let active = decode_granule(
                        hdr,
                        &mut bs,
                        &w.gr_info[igr * nch..],
                        nch,
                        &mut w.grbuf,
                        &mut w.scf,
                        &mut w.ist_pos,
                        &mut self.mdct_overlap,
                        &mut self.overlap_bands,
                    );
                    synth_granule(
                        &mut self.synth_hist,
                        &mut w.grbuf,
                        nch,
                        active,
                        &mut pcm[igr * 576 * nch..],
                    );
                }
            }
            reservoir_to_save(&bs)
        };

        // Keep the last (up to 511) bytes of main data for the next frame.
        // They may start in the old reservoir, which is moved down first.
        let from_head = bytes_have.saturating_sub(save_pos).min(save_len);
        let head_start = reserv_from + save_pos.min(bytes_have);
        self.reserv_buf
            .copy_within(head_start..head_start + from_head, 0);
        let tail_start = (save_pos + from_head).saturating_sub(bytes_have);
        let from_tail = save_len - from_head;
        self.reserv_buf[from_head..save_len]
            .copy_from_slice(&frame_main[tail_start..tail_start + from_tail]);
        self.reserv = save_len as i32;

        let samples = if success {
            hdr.frame_samples() as usize
        } else {
            0
        };
        (samples, info)
    }
}

#[allow(clippy::too_many_arguments)]
fn decode_granule(
    hdr: Header,
    bs: &mut BitReader,
    gr_info: &[GrInfo],
    nch: usize,
    grbuf: &mut [f32; 1152],
    scf: &mut [f32; 40],
    ist_pos: &mut [[u8; 39]; 2],
    mdct_overlap: &mut [[f32; 288]; 2],
    overlap_bands: &mut [u8; 2],
) -> [bool; 2] {
    let mut active = [false; 2];
    // Values of each channel that may be nonzero. grbuf is zeroed before each
    // granule, so everything after what Huffman decoding wrote is zero.
    let mut nonzero = [576usize; 2];
    for ch in 0..nch {
        let layer3gr_limit = bs.pos + gr_info[ch].part_23_length as i32;
        crate::timed!(
            1,
            layer3::decode_scalefactors(hdr, &mut ist_pos[ch], bs, &gr_info[ch], scf, ch)
        );
        nonzero[ch] = crate::timed!(
            2,
            layer3::huffman(
                &mut grbuf[576 * ch..],
                bs,
                &gr_info[ch],
                scf,
                layer3gr_limit,
            )
        );
    }
    if hdr.test_i_stereo() || hdr.is_ms_stereo() {
        // Stereo processing mixes the channels.
        let n = nonzero[0].max(nonzero[1]);
        nonzero = [n; 2];
    }

    crate::timed!(3, {
        if hdr.test_i_stereo() {
            layer3::intensity_stereo(grbuf, &mut ist_pos[1], gr_info, hdr);
        } else if hdr.is_ms_stereo() {
            layer3::midside_stereo(grbuf, 0, 576);
        }
    });

    for ch in 0..nch {
        let gr = &gr_info[ch];
        let mut aa_bands = 31;
        let n_long_bands =
            (if gr.mixed_block_flag != 0 { 2 } else { 0 }) << (hdr.my_sample_rate() == 2) as u32;

        crate::timed!(4, {
            if gr.n_short_sfb != 0 {
                aa_bands = n_long_bands as i32 - 1;
                // What follows this channel's grbuf in minimp3's scratch struct.
                let (ch0, ch1) = grbuf.split_at_mut(576);
                let (g, spill): (&mut [f32], &mut [f32]) = if ch == 0 {
                    (ch0, ch1)
                } else {
                    (ch1, &mut scf[..])
                };
                layer3::reorder(
                    &mut g[n_long_bands * 18..],
                    spill,
                    &gr.sfbtab()[gr.n_long_sfb as usize..],
                );
            }
        });
        // Bands that may be nonzero. The exact build does all the work
        // (skipped zeros could differ from minimp3's in sign).
        let mut bands = if cfg!(feature = "exact") || gr.n_short_sfb != 0 {
            32
        } else {
            nonzero[ch].div_ceil(18).min(32)
        };
        let g = &mut grbuf[576 * ch..576 * ch + 576];
        // Antialiasing mixes each band with the next.
        crate::timed!(4, layer3::antialias(g, aa_bands.min(bands as i32)));
        if bands > 0 {
            bands = (bands + 1).min(32);
        }
        // A band with zero input and zero overlap has zero output and overlap.
        let imdct_bands = bands.max(overlap_bands[ch] as usize);
        crate::timed!(5, {
            layer3::imdct_gr(
                g,
                &mut mdct_overlap[ch],
                gr.block_type,
                n_long_bands,
                imdct_bands,
            );
            layer3::change_sign(g, imdct_bands);
        });
        overlap_bands[ch] = bands as u8;
        active[ch] = imdct_bands > 0;
    }
    active
}

/// Which main data bytes to keep as the next frame's bit reservoir: (start, len).
fn reservoir_to_save(bs: &BitReader) -> (usize, usize) {
    let mut pos = ((bs.pos + 7) as u32 / 8) as i32;
    let mut remains = (bs.limit as u32 / 8) as i32 - pos;
    if remains > MAX_BITRESERVOIR_BYTES as i32 {
        pos += remains - MAX_BITRESERVOIR_BYTES as i32;
        remains = MAX_BITRESERVOIR_BYTES as i32;
    }
    (pos.max(0) as usize, remains.max(0) as usize)
}
