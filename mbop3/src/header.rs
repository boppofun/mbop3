//! MPEG audio frame header parsing and frame sync.

use crate::tables::{G_HZ, HALFRATE};

pub(crate) const HDR_SIZE: usize = 4;
const MAX_FREE_FORMAT_FRAME_SIZE: i32 = 2304;
const MAX_FRAME_SYNC_MATCHES: i32 = 10;

/// A 4 byte frame header.
#[derive(Clone, Copy, Default)]
pub(crate) struct Header(pub [u8; 4]);

impl Header {
    /// The header at the start of `h`, which must be at least 4 bytes.
    #[inline]
    pub fn at(h: &[u8]) -> Header {
        Header([h[0], h[1], h[2], h[3]])
    }

    pub fn is_mono(self) -> bool {
        self.0[3] & 0xC0 == 0xC0
    }
    pub fn is_ms_stereo(self) -> bool {
        self.0[3] & 0xE0 == 0x60
    }
    pub fn is_free_format(self) -> bool {
        self.0[2] & 0xF0 == 0
    }
    pub fn is_crc(self) -> bool {
        self.0[1] & 1 == 0
    }
    pub fn test_padding(self) -> bool {
        self.0[2] & 0x2 != 0
    }
    pub fn test_mpeg1(self) -> bool {
        self.0[1] & 0x8 != 0
    }
    pub fn test_not_mpeg25(self) -> bool {
        self.0[1] & 0x10 != 0
    }
    pub fn test_i_stereo(self) -> bool {
        self.0[3] & 0x10 != 0
    }
    pub fn test_ms_stereo(self) -> bool {
        self.0[3] & 0x20 != 0
    }
    pub fn layer(self) -> u8 {
        (self.0[1] >> 1) & 3
    }
    pub fn bitrate(self) -> u8 {
        self.0[2] >> 4
    }
    pub fn sample_rate(self) -> u8 {
        (self.0[2] >> 2) & 3
    }
    /// Sample rate index across MPEG versions: 0-2 MPEG-2.5, 3-5 MPEG-2, 6-8 MPEG-1.
    pub fn my_sample_rate(self) -> u8 {
        self.sample_rate() + (((self.0[1] >> 3) & 1) + ((self.0[1] >> 4) & 1)) * 3
    }
    pub fn is_frame_576(self) -> bool {
        self.0[1] & 14 == 2
    }
    pub fn is_layer_1(self) -> bool {
        self.0[1] & 6 == 6
    }

    pub fn valid(self) -> bool {
        let h = self.0;
        h[0] == 0xff
            && ((h[1] & 0xF0) == 0xf0 || (h[1] & 0xFE) == 0xe2)
            && self.layer() != 0
            && self.bitrate() != 15
            && self.sample_rate() != 3
    }

    /// Whether `other` is a valid header of the same stream format as `self`.
    pub fn compare(self, other: Header) -> bool {
        let (h1, h2) = (self.0, other.0);
        other.valid()
            && (h1[1] ^ h2[1]) & 0xFE == 0
            && (h1[2] ^ h2[2]) & 0x0C == 0
            && self.is_free_format() == other.is_free_format()
    }

    pub fn bitrate_kbps(self) -> u32 {
        2 * HALFRATE[self.test_mpeg1() as usize][self.layer() as usize - 1][self.bitrate() as usize]
            as u32
    }

    pub fn sample_rate_hz(self) -> u32 {
        G_HZ[self.sample_rate() as usize]
            >> (!self.test_mpeg1() as u32)
            >> (!self.test_not_mpeg25() as u32)
    }

    /// Samples per channel in a frame.
    pub fn frame_samples(self) -> u32 {
        if self.is_layer_1() {
            384
        } else {
            1152 >> (self.is_frame_576() as u32)
        }
    }

    pub fn frame_bytes(self, free_format_size: i32) -> i32 {
        let mut frame_bytes =
            (self.frame_samples() * self.bitrate_kbps() * 125 / self.sample_rate_hz()) as i32;
        if self.is_layer_1() {
            frame_bytes &= !3; // slot align
        }
        if frame_bytes != 0 {
            frame_bytes
        } else {
            free_format_size
        }
    }

    pub fn padding(self) -> i32 {
        if self.test_padding() {
            if self.is_layer_1() { 4 } else { 1 }
        } else {
            0
        }
    }
}

/// Whether at least one (and up to MAX_FRAME_SYNC_MATCHES) following frames
/// have a header matching the one at the start of `mp3`.
fn match_frame(mp3: &[u8], frame_bytes: i32) -> bool {
    let hdr = Header::at(mp3);
    let mut i: i32 = 0;
    for nmatch in 0..MAX_FRAME_SYNC_MATCHES {
        let h = Header::at(&mp3[i as usize..]);
        i += h.frame_bytes(frame_bytes) + h.padding();
        if i + HDR_SIZE as i32 > mp3.len() as i32 {
            return nmatch > 0;
        }
        if !hdr.compare(Header::at(&mp3[i as usize..])) {
            return false;
        }
    }
    true
}

/// Finds the first frame in `mp3`. Returns (offset, frame bytes including
/// padding), with frame bytes 0 (and offset `mp3.len()`) if none was found.
pub(crate) fn find_frame(mp3: &[u8], free_format_bytes: &mut i32) -> (i32, i32) {
    let mp3_bytes = mp3.len() as i32;
    let mut i: i32 = 0;
    while i < mp3_bytes - HDR_SIZE as i32 {
        let here = &mp3[i as usize..];
        let hdr = Header::at(here);
        if hdr.valid() {
            let mut frame_bytes = hdr.frame_bytes(*free_format_bytes);
            let mut frame_and_padding = frame_bytes + hdr.padding();

            let mut k = HDR_SIZE as i32;
            while frame_bytes == 0
                && k < MAX_FREE_FORMAT_FRAME_SIZE
                && i + 2 * k < mp3_bytes - HDR_SIZE as i32
            {
                if hdr.compare(Header::at(&here[k as usize..])) {
                    let fb = k - hdr.padding();
                    let nextfb = fb + Header::at(&here[k as usize..]).padding();
                    if !(i + k + nextfb + HDR_SIZE as i32 > mp3_bytes
                        || !hdr.compare(Header::at(&here[(k + nextfb) as usize..])))
                    {
                        frame_and_padding = k;
                        frame_bytes = fb;
                        *free_format_bytes = fb;
                    }
                }
                k += 1;
            }
            if (frame_bytes != 0
                && i + frame_and_padding <= mp3_bytes
                && match_frame(here, frame_bytes))
                || (i == 0 && frame_and_padding == mp3_bytes)
            {
                return (i, frame_and_padding);
            }
            *free_format_bytes = 0;
        }
        i += 1;
    }
    (mp3_bytes, 0)
}
