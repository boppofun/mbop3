//! Layer III side info, scalefactors, Huffman decoding, stereo processing and IMDCT.

use crate::bits::BitReader;
use crate::header::Header;
use crate::tables::*;

const SHORT_BLOCK_TYPE: u8 = 2;
const STOP_BLOCK_TYPE: u8 = 3;
const BITS_DEQUANTIZER_OUT: i32 = -1;
const MAX_SCF: i32 = 255 + BITS_DEQUANTIZER_OUT * 4 - 210;
const MAX_SCFI: i32 = (MAX_SCF + 3) & !3;

/// Side info of one granule of one channel.
#[derive(Clone, Copy)]
pub(crate) struct GrInfo {
    /// Scalefactor band widths, 0 terminated.
    pub sfbtab: &'static [u8],
    pub part_23_length: u16,
    pub big_values: u16,
    pub scalefac_compress: u16,
    pub global_gain: u8,
    pub block_type: u8,
    pub mixed_block_flag: u8,
    pub n_long_sfb: u8,
    pub n_short_sfb: u8,
    pub table_select: [u8; 3],
    pub region_count: [u8; 3],
    pub subblock_gain: [u8; 3],
    pub preflag: u8,
    pub scalefac_scale: u8,
    pub count1_table: u8,
    pub scfsi: u8,
}

impl GrInfo {
    pub const ZERO: GrInfo = GrInfo {
        sfbtab: &[],
        part_23_length: 0,
        big_values: 0,
        scalefac_compress: 0,
        global_gain: 0,
        block_type: 0,
        mixed_block_flag: 0,
        n_long_sfb: 0,
        n_short_sfb: 0,
        table_select: [0; 3],
        region_count: [0; 3],
        subblock_gain: [0; 3],
        preflag: 0,
        scalefac_scale: 0,
        count1_table: 0,
        scfsi: 0,
    };
}

/// Reads the side info of all granules. Returns main_data_begin, or -1 if invalid.
pub(crate) fn read_side_info(bs: &mut BitReader, grs: &mut [GrInfo; 4], hdr: Header) -> i32 {
    let mut scfsi: u32 = 0;
    let main_data_begin: i32;
    let mut part_23_sum: i32 = 0;
    let mut sr_idx = hdr.my_sample_rate() as usize;
    sr_idx -= (sr_idx != 0) as usize;
    let mut gr_count = if hdr.is_mono() { 1 } else { 2 };

    if hdr.test_mpeg1() {
        gr_count *= 2;
        main_data_begin = bs.get_bits(9) as i32;
        scfsi = bs.get_bits(7 + gr_count);
    } else {
        main_data_begin = (bs.get_bits(8 + gr_count) >> gr_count) as i32;
    }

    for gr in grs.iter_mut().take(gr_count as usize) {
        if hdr.is_mono() {
            scfsi <<= 4;
        }
        gr.part_23_length = bs.get_bits(12) as u16;
        part_23_sum += gr.part_23_length as i32;
        gr.big_values = bs.get_bits(9) as u16;
        if gr.big_values > 288 {
            return -1;
        }
        gr.global_gain = bs.get_bits(8) as u8;
        gr.scalefac_compress = bs.get_bits(if hdr.test_mpeg1() { 4 } else { 9 }) as u16;
        gr.sfbtab = &G_SCF_LONG[sr_idx];
        gr.n_long_sfb = 22;
        gr.n_short_sfb = 0;
        let mut tables: u32;
        if bs.get_bits(1) != 0 {
            gr.block_type = bs.get_bits(2) as u8;
            if gr.block_type == 0 {
                return -1;
            }
            gr.mixed_block_flag = bs.get_bits(1) as u8;
            gr.region_count[0] = 7;
            gr.region_count[1] = 255;
            if gr.block_type == SHORT_BLOCK_TYPE {
                scfsi &= 0x0F0F;
                if gr.mixed_block_flag == 0 {
                    gr.region_count[0] = 8;
                    gr.sfbtab = &G_SCF_SHORT[sr_idx];
                    gr.n_long_sfb = 0;
                    gr.n_short_sfb = 39;
                } else {
                    gr.sfbtab = &G_SCF_MIXED[sr_idx];
                    gr.n_long_sfb = if hdr.test_mpeg1() { 8 } else { 6 };
                    gr.n_short_sfb = 30;
                }
            }
            tables = bs.get_bits(10);
            tables <<= 5;
            gr.subblock_gain[0] = bs.get_bits(3) as u8;
            gr.subblock_gain[1] = bs.get_bits(3) as u8;
            gr.subblock_gain[2] = bs.get_bits(3) as u8;
        } else {
            gr.block_type = 0;
            gr.mixed_block_flag = 0;
            tables = bs.get_bits(15);
            gr.region_count[0] = bs.get_bits(4) as u8;
            gr.region_count[1] = bs.get_bits(3) as u8;
            gr.region_count[2] = 255;
        }
        gr.table_select[0] = (tables >> 10) as u8;
        gr.table_select[1] = ((tables >> 5) & 31) as u8;
        gr.table_select[2] = (tables & 31) as u8;
        gr.preflag = if hdr.test_mpeg1() {
            bs.get_bits(1) as u8
        } else {
            (gr.scalefac_compress >= 500) as u8
        };
        gr.scalefac_scale = bs.get_bits(1) as u8;
        gr.count1_table = bs.get_bits(1) as u8;
        gr.scfsi = ((scfsi >> 12) & 15) as u8;
        scfsi <<= 4;
    }

    if part_23_sum + bs.pos > bs.limit + main_data_begin * 8 {
        return -1;
    }
    main_data_begin
}

fn read_scalefactors(
    scf: &mut [u8; 40],
    ist_pos: &mut [u8; 39],
    scf_size: &[u8; 4],
    scf_count: &[u8],
    bs: &mut BitReader,
    mut scfsi: i32,
) {
    let mut off = 0;
    let mut i = 0;
    while i < 4 && scf_count[i] != 0 {
        let cnt = scf_count[i] as usize;
        if scfsi & 8 != 0 {
            scf[off..off + cnt].copy_from_slice(&ist_pos[off..off + cnt]);
        } else {
            let bits = scf_size[i] as i32;
            if bits == 0 {
                scf[off..off + cnt].fill(0);
                ist_pos[off..off + cnt].fill(0);
            } else {
                let max_scf = if scfsi < 0 { (1 << bits) - 1 } else { -1 };
                for k in 0..cnt {
                    let s = bs.get_bits(bits) as i32;
                    ist_pos[off + k] = if s == max_scf { 255 } else { s as u8 };
                    scf[off + k] = s as u8;
                }
            }
        }
        off += cnt;
        i += 1;
        scfsi *= 2;
    }
    scf[off] = 0;
    scf[off + 1] = 0;
    scf[off + 2] = 0;
}

fn ldexp_q2(mut y: f32, mut exp_q2: i32) -> f32 {
    loop {
        let e = exp_q2.min(30 * 4);
        y *= G_EXPFRAC[(e & 3) as usize] * ((1i32 << 30 >> (e >> 2)) as f32);
        exp_q2 -= e;
        if exp_q2 <= 0 {
            break;
        }
    }
    y
}

pub(crate) fn decode_scalefactors(
    hdr: Header,
    ist_pos: &mut [u8; 39],
    bs: &mut BitReader,
    gr: &GrInfo,
    scf: &mut [f32; 40],
    ch: usize,
) {
    let mut scf_partition: &[u8] =
        &G_SCF_PARTITIONS[(gr.n_short_sfb != 0) as usize + (gr.n_long_sfb == 0) as usize];
    let mut scf_size = [0u8; 4];
    let mut iscf = [0u8; 40];
    let scf_shift = gr.scalefac_scale as i32 + 1;
    let mut scfsi = gr.scfsi as i32;

    if hdr.test_mpeg1() {
        let part = G_SCFC_DECODE[gr.scalefac_compress as usize];
        scf_size[0] = part >> 2;
        scf_size[1] = part >> 2;
        scf_size[2] = part & 3;
        scf_size[3] = part & 3;
    } else {
        let ist = (hdr.test_i_stereo() && ch != 0) as usize;
        let mut sfc = (gr.scalefac_compress >> ist) as i32;
        let mut k = ist * 3 * 4;
        while sfc >= 0 {
            let mut modprod = 1;
            for i in (0..4).rev() {
                scf_size[i] = (sfc / modprod % G_MOD[k + i] as i32) as u8;
                modprod *= G_MOD[k + i] as i32;
            }
            sfc -= modprod;
            k += 4;
        }
        scf_partition = &scf_partition[k..];
        scfsi = -16;
    }
    read_scalefactors(&mut iscf, ist_pos, &scf_size, scf_partition, bs, scfsi);

    let n_long = gr.n_long_sfb as usize;
    if gr.n_short_sfb != 0 {
        let sh = 3 - scf_shift;
        for i in (0..gr.n_short_sfb as usize).step_by(3) {
            for k in 0..3 {
                let j = n_long + i + k;
                iscf[j] = (iscf[j] as i32 + ((gr.subblock_gain[k] as i32) << sh)) as u8;
            }
        }
    } else if gr.preflag != 0 {
        for (i, p) in G_PREAMP.iter().enumerate() {
            iscf[11 + i] = iscf[11 + i].wrapping_add(*p);
        }
    }

    let gain_exp = gr.global_gain as i32 + BITS_DEQUANTIZER_OUT * 4
        - 210
        - if hdr.is_ms_stereo() { 2 } else { 0 };
    let gain = ldexp_q2((1 << (MAX_SCFI / 4)) as f32, MAX_SCFI - gain_exp);
    for i in 0..n_long + gr.n_short_sfb as usize {
        scf[i] = ldexp_q2(gain, (iscf[i] as i32) << scf_shift);
    }
}

fn pow_43(mut x: i32) -> f32 {
    let mut mult = 256;
    if x < 129 {
        return G_POW43[(16 + x) as usize];
    }
    if x < 1024 {
        mult = 16;
        x <<= 3;
    }
    let sign = (2 * x) & 64;
    let frac = ((x & 63) - sign) as f32 / ((x & !63) + sign) as f32;
    G_POW43[(16 + ((x + sign) >> 6)) as usize]
        * (1.0 + frac * ((4.0 / 3.0) + frac * (2.0 / 9.0)))
        * mult as f32
}

/// Huffman decodes and dequantizes one granule of one channel into `dst`.
///
/// Reads ahead of the bit reader's limit (like minimp3); bytes past the end
/// of the buffer read as zero.
pub(crate) fn huffman(
    dst: &mut [f32],
    bs: &mut BitReader,
    gr: &GrInfo,
    scf: &[f32; 40],
    layer3gr_limit: i32,
) {
    let (head, tail) = bs.split_from(0);
    let byte = |i: usize| match head.get(i) {
        Some(b) => *b as u32,
        None => tail.get(i - head.len()).copied().unwrap_or(0) as u32,
    };
    let mut one = 0.0f32;
    let mut ireg = 0;
    let mut big_val_cnt = gr.big_values as i32;
    let sfb = gr.sfbtab;
    let mut sfb_i = 0;
    let mut scf_i = 0;
    let mut next = (bs.pos / 8) as usize;
    let mut cache: u32 = (((byte(next) * 256 + byte(next + 1)) * 256 + byte(next + 2)) * 256
        + byte(next + 3))
        << (bs.pos & 7);
    let mut sh: i32 = (bs.pos & 7) - 8;
    next += 4;
    let mut d = 0;

    macro_rules! peek {
        ($n:expr) => {
            cache.wrapping_shr(32 - ($n) as u32)
        };
    }
    macro_rules! flush {
        ($n:expr) => {{
            let n = ($n) as u32;
            cache = cache.wrapping_shl(n);
            sh += n as i32;
        }};
    }
    macro_rules! check_bits {
        () => {
            while sh >= 0 {
                cache |= byte(next) << sh;
                next += 1;
                sh -= 8;
            }
        };
    }

    while big_val_cnt > 0 {
        let tab_num = gr.table_select[ireg] as usize;
        let mut sfb_cnt = gr.region_count[ireg] as i32;
        ireg += 1;
        let codebook = &TABS[TABINDEX[tab_num] as usize..];
        let linbits = G_LINBITS[tab_num] as i32;
        loop {
            let np = sfb[sfb_i] as i32 / 2;
            sfb_i += 1;
            let mut pairs_to_decode = big_val_cnt.min(np);
            one = scf[scf_i];
            scf_i += 1;
            loop {
                let mut w = 5;
                let mut leaf = codebook[peek!(w) as usize] as i32;
                while leaf < 0 {
                    flush!(w);
                    w = leaf & 7;
                    leaf = codebook[(peek!(w) as i32 - (leaf >> 3)) as usize] as i32;
                }
                flush!(leaf >> 8);

                for _ in 0..2 {
                    let mut lsb = leaf & 0x0F;
                    if linbits != 0 && lsb == 15 {
                        lsb += peek!(linbits) as i32;
                        flush!(linbits);
                        check_bits!();
                        dst[d] = one * pow_43(lsb) * if (cache as i32) < 0 { -1.0 } else { 1.0 };
                    } else {
                        dst[d] = G_POW43[(16 + lsb - 16 * (cache >> 31) as i32) as usize] * one;
                    }
                    flush!(if lsb != 0 { 1 } else { 0 });
                    d += 1;
                    leaf >>= 4;
                }
                check_bits!();
                pairs_to_decode -= 1;
                if pairs_to_decode == 0 {
                    break;
                }
            }
            big_val_cnt -= np;
            if big_val_cnt <= 0 {
                break;
            }
            sfb_cnt -= 1;
            if sfb_cnt < 0 {
                break;
            }
        }
    }

    let codebook_count1: &[u8] = if gr.count1_table != 0 { &TAB33 } else { &TAB32 };
    let mut np = 1 - big_val_cnt;
    loop {
        let mut leaf = codebook_count1[peek!(4) as usize] as i32;
        if leaf & 8 == 0 {
            leaf = codebook_count1
                [((leaf >> 3) as u32 + (cache << 4).wrapping_shr(32 - (leaf & 3) as u32)) as usize]
                as i32;
        }
        flush!(leaf & 7);
        if (next as i32) * 8 - 24 + sh > layer3gr_limit {
            break;
        }
        macro_rules! reload_scalefactor {
            () => {
                np -= 1;
                if np == 0 {
                    np = sfb[sfb_i] as i32 / 2;
                    sfb_i += 1;
                    if np == 0 {
                        break;
                    }
                    one = scf[scf_i];
                    scf_i += 1;
                }
            };
        }
        macro_rules! deq_count1 {
            ($s:expr) => {
                if leaf & (128 >> $s) != 0 {
                    dst[d + $s] = if (cache as i32) < 0 { -one } else { one };
                    flush!(1);
                }
            };
        }
        reload_scalefactor!();
        deq_count1!(0);
        deq_count1!(1);
        reload_scalefactor!();
        deq_count1!(2);
        deq_count1!(3);
        check_bits!();
        d += 4;
    }

    bs.pos = layer3gr_limit;
}

/// Mid/side stereo on `n` values starting at `o` (left) and `o + 576` (right).
pub(crate) fn midside_stereo(g: &mut [f32], o: usize, n: usize) {
    for i in o..o + n {
        let a = g[i];
        let b = g[i + 576];
        g[i] = a + b;
        g[i + 576] = a - b;
    }
}

fn intensity_stereo_band(g: &mut [f32], o: usize, n: usize, kl: f32, kr: f32) {
    for i in o..o + n {
        g[i + 576] = g[i] * kr;
        g[i] *= kl;
    }
}

fn stereo_top_band(g: &[f32], sfb: &[u8], nbands: usize, max_band: &mut [i32; 3]) {
    *max_band = [-1; 3];
    let mut r = 576;
    for (i, &w) in sfb.iter().enumerate().take(nbands) {
        let w = w as usize;
        let mut k = 0;
        while k < w {
            if g[r + k] != 0.0 || g[r + k + 1] != 0.0 {
                max_band[i % 3] = i as i32;
                break;
            }
            k += 2;
        }
        r += w;
    }
}

fn stereo_process(
    g: &mut [f32],
    ist_pos: &[u8; 39],
    sfb: &[u8],
    hdr: Header,
    max_band: &[i32; 3],
    mpeg2_sh: u32,
) {
    let max_pos: u32 = if hdr.test_mpeg1() { 7 } else { 64 };
    let mut o = 0;
    let mut i = 0;
    while sfb[i] != 0 {
        let w = sfb[i] as usize;
        let ipos = ist_pos[i] as u32;
        if i as i32 > max_band[i % 3] && ipos < max_pos {
            let s = if hdr.test_ms_stereo() {
                1.41421356
            } else {
                1.0
            };
            let (kl, kr);
            if hdr.test_mpeg1() {
                kl = G_PAN[2 * ipos as usize];
                kr = G_PAN[2 * ipos as usize + 1];
            } else {
                let k = ldexp_q2(1.0, (((ipos + 1) >> 1) << mpeg2_sh) as i32);
                if ipos & 1 != 0 {
                    kl = k;
                    kr = 1.0;
                } else {
                    kl = 1.0;
                    kr = k;
                }
            }
            intensity_stereo_band(g, o, w, kl * s, kr * s);
        } else if hdr.test_ms_stereo() {
            midside_stereo(g, o, w);
        }
        o += w;
        i += 1;
    }
}

/// `gr` is the side info of both channels of the granule.
pub(crate) fn intensity_stereo(g: &mut [f32], ist_pos: &mut [u8; 39], gr: &[GrInfo], hdr: Header) {
    let mut max_band = [0i32; 3];
    let n_sfb = gr[0].n_long_sfb as usize + gr[0].n_short_sfb as usize;
    let max_blocks = if gr[0].n_short_sfb != 0 { 3 } else { 1 };

    stereo_top_band(g, gr[0].sfbtab, n_sfb, &mut max_band);
    if gr[0].n_long_sfb != 0 {
        let m = max_band[0].max(max_band[1]).max(max_band[2]);
        max_band = [m; 3];
    }
    for i in 0..max_blocks {
        let default_pos = if hdr.test_mpeg1() { 3 } else { 0 };
        let itop = n_sfb - max_blocks + i;
        let prev = itop - max_blocks;
        ist_pos[itop] = if max_band[i] >= prev as i32 {
            default_pos
        } else {
            ist_pos[prev]
        };
    }
    stereo_process(
        g,
        ist_pos,
        gr[0].sfbtab,
        hdr,
        &max_band,
        (gr[1].scalefac_compress & 1) as u32,
    );
}

/// Reorders short block values from (window, frequency) to (frequency, window) order.
pub(crate) fn reorder(g: &mut [f32], scratch: &mut [f32], sfb: &[u8]) {
    let mut src = 0;
    let mut dst = 0;
    let mut i = 0;
    loop {
        let len = sfb[i] as usize;
        if len == 0 {
            break;
        }
        for k in 0..len {
            scratch[dst] = g[src + k];
            scratch[dst + 1] = g[src + k + len];
            scratch[dst + 2] = g[src + k + 2 * len];
            dst += 3;
        }
        src += 3 * len;
        i += 3;
    }
    g[..dst].copy_from_slice(&scratch[..dst]);
}

pub(crate) fn antialias(g: &mut [f32], nbands: i32) {
    for band in 0..nbands.max(0) as usize {
        let b = band * 18;
        for i in 0..8 {
            let u = g[b + 18 + i];
            let d = g[b + 17 - i];
            g[b + 18 + i] = u * G_AA[0][i] - d * G_AA[1][i];
            g[b + 17 - i] = u * G_AA[1][i] + d * G_AA[0][i];
        }
    }
}

fn dct3_9(y: &mut [f32; 9]) {
    let mut s0 = y[0];
    let mut s2 = y[2];
    let mut s4 = y[4];
    let mut s6 = y[6];
    let mut s8 = y[8];
    let mut t0 = s0 + s6 * 0.5;
    s0 -= s6;
    let mut t4 = (s4 + s2) * 0.93969262;
    let mut t2 = (s8 + s2) * 0.76604444;
    s6 = (s4 - s8) * 0.17364818;
    s4 += s8 - s2;

    s2 = s0 - s4 * 0.5;
    y[4] = s4 + s0;
    s8 = t0 - t2 + s6;
    s0 = t0 - t4 + t2;
    s4 = t0 + t4 - s6;

    let mut s1 = y[1];
    let mut s3 = y[3];
    let mut s5 = y[5];
    let mut s7 = y[7];

    s3 *= 0.86602540;
    t0 = (s5 + s1) * 0.98480775;
    t4 = (s5 - s7) * 0.34202014;
    t2 = (s1 + s7) * 0.64278761;
    s1 = (s1 - s5 - s7) * 0.86602540;

    s5 = t0 - s3 - t2;
    s7 = t4 - s3 - t0;
    s3 = t4 + s3 - t2;

    y[0] = s4 - s7;
    y[1] = s2 + s1;
    y[2] = s0 - s3;
    y[3] = s8 + s5;
    y[5] = s8 - s5;
    y[6] = s0 + s3;
    y[7] = s2 - s1;
    y[8] = s4 + s7;
}

fn imdct36(g: &mut [f32], overlap: &mut [f32], window: &[f32; 18], nbands: usize) {
    for j in 0..nbands {
        let g = &mut g[j * 18..j * 18 + 18];
        let overlap = &mut overlap[j * 9..j * 9 + 9];
        let mut co = [0f32; 9];
        let mut si = [0f32; 9];
        co[0] = -g[0];
        si[0] = g[17];
        for i in 0..4 {
            si[8 - 2 * i] = g[4 * i + 1] - g[4 * i + 2];
            co[1 + 2 * i] = g[4 * i + 1] + g[4 * i + 2];
            si[7 - 2 * i] = g[4 * i + 4] - g[4 * i + 3];
            co[2 + 2 * i] = -(g[4 * i + 3] + g[4 * i + 4]);
        }
        dct3_9(&mut co);
        dct3_9(&mut si);

        si[1] = -si[1];
        si[3] = -si[3];
        si[5] = -si[5];
        si[7] = -si[7];

        for i in 0..9 {
            let ovl = overlap[i];
            let sum = co[i] * G_TWID9[9 + i] + si[i] * G_TWID9[i];
            overlap[i] = co[i] * G_TWID9[i] - si[i] * G_TWID9[9 + i];
            g[i] = ovl * window[i] - sum * window[9 + i];
            g[17 - i] = ovl * window[9 + i] + sum * window[i];
        }
    }
}

fn idct3(x0: f32, x1: f32, x2: f32) -> [f32; 3] {
    let m1 = x1 * 0.86602540;
    let a1 = x0 - x2 * 0.5;
    [a1 + m1, x0 + x2, a1 - m1]
}

fn imdct12(x: &[f32], dst: &mut [f32], overlap: &mut [f32]) {
    let co = idct3(-x[0], x[6] + x[3], x[12] + x[9]);
    let mut si = idct3(x[15], x[12] - x[9], x[6] - x[3]);
    si[1] = -si[1];

    for i in 0..3 {
        let ovl = overlap[i];
        let sum = co[i] * G_TWID3[3 + i] + si[i] * G_TWID3[i];
        overlap[i] = co[i] * G_TWID3[i] - si[i] * G_TWID3[3 + i];
        dst[i] = ovl * G_TWID3[2 - i] - sum * G_TWID3[5 - i];
        dst[5 - i] = ovl * G_TWID3[5 - i] + sum * G_TWID3[2 - i];
    }
}

fn imdct_short(g: &mut [f32], overlap: &mut [f32], nbands: usize) {
    for j in 0..nbands {
        let g = &mut g[j * 18..j * 18 + 18];
        let overlap = &mut overlap[j * 9..j * 9 + 9];
        let mut tmp = [0f32; 18];
        tmp.copy_from_slice(g);
        g[..6].copy_from_slice(&overlap[..6]);
        imdct12(&tmp, &mut g[6..12], &mut overlap[6..]);
        imdct12(&tmp[1..], &mut g[12..18], &mut overlap[6..]);
        let (ov_out, ov_state) = overlap.split_at_mut(6);
        imdct12(&tmp[2..], ov_out, ov_state);
    }
}

pub(crate) fn change_sign(g: &mut [f32]) {
    let mut base = 18;
    for _ in (0..32).step_by(2) {
        for i in (1..18).step_by(2) {
            g[base + i] = -g[base + i];
        }
        base += 36;
    }
}

pub(crate) fn imdct_gr(
    g: &mut [f32],
    overlap: &mut [f32; 288],
    block_type: u8,
    n_long_bands: usize,
) {
    if n_long_bands > 0 {
        imdct36(g, overlap, &G_MDCT_WINDOW[0], n_long_bands);
    }
    let g = &mut g[18 * n_long_bands..];
    let overlap = &mut overlap[9 * n_long_bands..];
    if block_type == SHORT_BLOCK_TYPE {
        imdct_short(g, overlap, 32 - n_long_bands);
    } else {
        imdct36(
            g,
            overlap,
            &G_MDCT_WINDOW[(block_type == STOP_BLOCK_TYPE) as usize],
            32 - n_long_bands,
        );
    }
}
