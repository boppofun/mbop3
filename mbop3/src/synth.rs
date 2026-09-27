//! Polyphase synthesis filterbank.

use crate::Sample;
use crate::tables::{G_SEC, G_WIN};

fn dct_ii(g: &mut [f32; 576], n: usize) {
    for k in 0..n {
        let mut t = [[0f32; 8]; 4];
        for i in 0..8 {
            let x0 = g[k + i * 18];
            let x1 = g[k + (15 - i) * 18];
            let x2 = g[k + (16 + i) * 18];
            let x3 = g[k + (31 - i) * 18];
            let t0 = x0 + x3;
            let t1 = x1 + x2;
            let t2 = (x1 - x2) * G_SEC[3 * i];
            let t3 = (x0 - x3) * G_SEC[3 * i + 1];
            t[0][i] = t0 + t1;
            t[1][i] = (t0 - t1) * G_SEC[3 * i + 2];
            t[2][i] = t3 + t2;
            t[3][i] = (t3 - t2) * G_SEC[3 * i + 2];
        }
        for x in t.iter_mut() {
            let (mut x0, mut x1, mut x2, mut x3, mut x4, mut x5, mut x6, mut x7) =
                (x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]);
            let mut xt = x0 - x7;
            x0 += x7;
            x7 = x1 - x6;
            x1 += x6;
            x6 = x2 - x5;
            x2 += x5;
            x5 = x3 - x4;
            x3 += x4;
            x4 = x0 - x3;
            x0 += x3;
            x3 = x1 - x2;
            x1 += x2;
            x[0] = x0 + x1;
            x[4] = (x0 - x1) * 0.70710677;
            x5 += x6;
            x6 = (x6 + x7) * 0.70710677;
            x7 += xt;
            x3 = (x3 + x4) * 0.70710677;
            x5 -= x7 * 0.198912367; // rotate by PI/8
            x7 += x5 * 0.382683432;
            x5 -= x7 * 0.198912367;
            x0 = xt - x6;
            xt += x6;
            x[1] = (xt + x7) * 0.50979561;
            x[2] = (x4 + x3) * 0.54119611;
            x[3] = (x0 - x5) * 0.60134488;
            x[5] = (x0 + x5) * 0.89997619;
            x[6] = (x4 - x3) * 1.30656302;
            x[7] = (xt - x7) * 2.56291556;
        }
        let mut y = k;
        for i in 0..7 {
            g[y] = t[0][i];
            g[y + 18] = t[2][i] + t[3][i] + t[3][i + 1];
            g[y + 2 * 18] = t[1][i] + t[1][i + 1];
            g[y + 3 * 18] = t[2][i + 1] + t[3][i] + t[3][i + 1];
            y += 4 * 18;
        }
        g[y] = t[0][7];
        g[y + 18] = t[2][7] + t[3][7];
        g[y + 2 * 18] = t[1][7];
        g[y + 3 * 18] = t[3][7];
    }
}

/// Rows of 64 floats in the synthesis history ring buffer.
///
/// minimp3 keeps 15 rows of history and appends a granule's 18 new rows in a
/// 33 row scratch buffer, then copies the last 15 back. A ring of exactly 18
/// rows holds the same values without the copies: every access to row `r`
/// (numbered from the start of the granule, 0..33) goes to row `r % 18`,
/// which is the same physical row in every granule. At most 17 rows are live
/// at a time.
///
/// In mono minimp3 only carries the left (even) columns over to the next
/// granule, so the right channel's history is kept from the last stereo
/// granule (which matters if a stream switches to stereo). Not writing the
/// right columns in mono reproduces that exactly.
pub(crate) const HIST_ROWS: usize = 18;

/// The synthesis history: `HIST_ROWS` rows of 16 groups of 4 columns (left
/// even band, right even, left odd, right odd).
pub(crate) type Hist = [f32; HIST_ROWS * 64];

/// Column `c` of the last group of 4 columns, over rows `m0..m0 + 15` of `rows`.
#[inline(never)]
fn synth_pair<S: Sample>(
    out: &mut [S],
    p: usize,
    nch: usize,
    h: &Hist,
    rows: &[usize; 17],
    m0: usize,
    c: usize,
) {
    let z = |m: usize, c: usize| h[rows[m0 + m] + 60 + c];
    let mut a;
    a = (z(14, c) - z(0, c)) * 29.0;
    a += (z(1, c) + z(13, c)) * 213.0;
    a += (z(12, c) - z(2, c)) * 459.0;
    a += (z(3, c) + z(11, c)) * 2037.0;
    a += (z(10, c) - z(4, c)) * 5153.0;
    a += (z(5, c) + z(9, c)) * 6574.0;
    a += (z(8, c) - z(6, c)) * 37489.0;
    a += z(7, c) * 75038.0;
    out[p] = S::from_synth(a);

    let c = c + 2;
    a = z(14, c) * 104.0;
    a += z(12, c) * 1567.0;
    a += z(10, c) * 9727.0;
    a += z(8, c) * 64019.0;
    a += z(6, c) * -9975.0;
    a += z(4, c) * -45.0;
    a += z(2, c) * 146.0;
    a += z(0, c) * -5.0;
    out[p + 16 * nch] = S::from_synth(a);
}

/// The window sums for history columns `c` and `c + 2` (one channel's even
/// and odd band) of the group of 4 columns at `c & !3`: (a_c, b_c, a_c2, b_c2).
///
/// minimp3 starts with (k = 0)
///   b = z*w1 + y*w0; a = z*w0 - y*w1
/// then for k = 1..8 adds b += z*w1 + y*w0 and, for even k, a += z*w0 - y*w1
/// or, for odd k, a += y*w1 - z*w0, which is exactly a -= z*w0 - y*w1.
#[cfg(feature = "exact")]
#[inline(always)]
fn accumulate(h: &Hist, rows: &[usize; 17], w: &[f32; 16], c: usize) -> (f32, f32, f32, f32) {
    let load = |m: usize| -> (f32, f32) {
        let v: &[f32; 3] = h[rows[m] + c..rows[m] + c + 3].try_into().unwrap();
        (v[0], v[2])
    };
    let ((z0, z2), (y0, y2)) = (load(15), load(0));
    let (w0, w1) = (w[0], w[1]);
    let (mut b0, mut a0) = (z0 * w1 + y0 * w0, z0 * w0 - y0 * w1);
    let (mut b2, mut a2) = (z2 * w1 + y2 * w0, z2 * w0 - y2 * w1);
    // The four chains are independent; their operations are interleaved so a
    // simple in-order FPU (the ESP32-S3's) isn't stalled waiting on each result.
    macro_rules! step {
        ($k:expr, $sub:tt) => {{
            let ((z0, z2), (y0, y2)) = (load(15 - $k), load($k));
            let (w0, w1) = (w[2 * $k], w[2 * $k + 1]);
            let (p0, p2, q0, q2) = (z0 * w1, z2 * w1, z0 * w0, z2 * w0);
            let (s0, s2) = (p0 + y0 * w0, p2 + y2 * w0);
            let (t0, t2) = (q0 - y0 * w1, q2 - y2 * w1);
            b0 += s0;
            b2 += s2;
            a0 $sub t0;
            a2 $sub t2;
        }};
    }
    let mut k = 1;
    loop {
        step!(k, -=); // odd k
        if k == 7 {
            return (a0, b0, a2, b2);
        }
        step!(k + 1, +=); // even k
        k += 2;
    }
}

/// The window sums for history columns `c` and `c + 2` (one channel's even
/// and odd band) of the group of 4 columns at `c & !3`: (a_c, b_c, a_c2, b_c2).
///
/// Same sums as minimp3 (see the "exact" version), but each is split into two
/// independent multiply-accumulate chains (z and y terms) that are added at
/// the end. The ESP32-S3's FPU has ~4 cycles of latency, and minimp3's
/// `b += z*w1 + y*w0` makes every operation wait for the previous one.
#[cfg(not(feature = "exact"))]
#[inline(always)]
fn accumulate(h: &Hist, rows: &[usize; 17], w: &[f32; 16], c: usize) -> (f32, f32, f32, f32) {
    const LEN: usize = HIST_ROWS * 64;
    let load = |r: usize| -> (f32, f32) {
        let v: &[f32; 3] = h[r..r + 3].try_into().unwrap();
        (v[0], v[2])
    };
    // Offsets of rows 15 - k (z) and k (y), stepped through the ring.
    let mut rz = rows[15] + c;
    let mut ry = rows[0] + c;
    let ((z0, z2), (y0, y2)) = (load(rz), load(ry));
    let (w0, w1) = (w[0], w[1]);
    let (mut bz0, mut by0, mut az0, mut ay0) = (z0 * w1, y0 * w0, z0 * w0, -(y0 * w1));
    let (mut bz2, mut by2, mut az2, mut ay2) = (z2 * w1, y2 * w0, z2 * w0, -(y2 * w1));
    macro_rules! step {
        ($k:expr, $zsign:tt, $ysign:tt) => {{
            rz = if rz >= 64 { rz - 64 } else { rz + LEN - 64 };
            ry = if ry + 64 >= LEN { ry + 64 - LEN } else { ry + 64 };
            let ((z0, z2), (y0, y2)) = (load(rz), load(ry));
            let (w0, w1) = (w[2 * $k], w[2 * $k + 1]);
            bz0 += z0 * w1;
            by0 += y0 * w0;
            az0 $zsign z0 * w0;
            ay0 $ysign y0 * w1;
            bz2 += z2 * w1;
            by2 += y2 * w0;
            az2 $zsign z2 * w0;
            ay2 $ysign y2 * w1;
        }};
    }
    let mut k = 1;
    loop {
        step!(k, -=, +=); // odd k: a -= z*w0 - y*w1
        if k == 7 {
            return (az0 + ay0, bz0 + by0, az2 + ay2, bz2 + by2);
        }
        step!(k + 1, +=, -=); // even k: a += z*w0 - y*w1
        k += 2;
    }
}

/// The window sums of all 4 columns at `q` (stereo): (a, b), in exactly
/// minimp3's order (see [`accumulate`]). With 8 independent sums there is
/// enough independent work to cover the FPU's latency without splitting them.
#[inline(always)]
fn accumulate4(h: &Hist, rows: &[usize; 17], w: &[f32; 16], q: usize) -> ([f32; 4], [f32; 4]) {
    const LEN: usize = HIST_ROWS * 64;
    let mut rz = rows[15] + q;
    let mut ry = rows[0] + q;
    let (mut a, mut b) = ([0f32; 4], [0f32; 4]);
    {
        let vz: &[f32; 4] = h[rz..rz + 4].try_into().unwrap();
        let vy: &[f32; 4] = h[ry..ry + 4].try_into().unwrap();
        let (w0, w1) = (w[0], w[1]);
        macro_rules! col {
            ($j:expr) => {
                b[$j] = vz[$j] * w1 + vy[$j] * w0;
                a[$j] = vz[$j] * w0 - vy[$j] * w1;
            };
        }
        col!(0);
        col!(1);
        col!(2);
        col!(3);
    }
    // rustfmt misindents the nested macro.
    #[rustfmt::skip]
    macro_rules! step {
        ($k:expr, $sub:tt) => {{
            rz = if rz >= 64 { rz - 64 } else { rz + LEN - 64 };
            ry = if ry + 64 >= LEN {
                ry + 64 - LEN
            } else {
                ry + 64
            };
            let vz: &[f32; 4] = h[rz..rz + 4].try_into().unwrap();
            let vy: &[f32; 4] = h[ry..ry + 4].try_into().unwrap();
            let (w0, w1) = (w[2 * $k], w[2 * $k + 1]);
            macro_rules! col {
                ($j:expr) => {
                    b[$j] += vz[$j] * w1 + vy[$j] * w0;
                    a[$j] $sub vz[$j] * w0 - vy[$j] * w1;
                };
            }
            col!(0);
            col!(1);
            col!(2);
            col!(3);
        }};
    }
    let mut k = 1;
    loop {
        step!(k, -=); // odd k: a += y*w1 - z*w0, i.e. a -= z*w0 - y*w1
        if k == 7 {
            return (a, b);
        }
        step!(k + 1, +=);
        k += 2;
    }
}

/// Synthesizes 2 x 32 output samples per channel from subband sample rows
/// `xl` and `xl + 1` of `g` into `out` (64 samples per channel).
#[inline(never)]
fn synth<S: Sample, const NCH: usize>(g: &[f32; 1152], xl: usize, out: &mut [S], h: &mut Hist) {
    let stereo = NCH == 2;
    let xr = xl + 576 * (NCH - 1);
    let (dl, dr) = (0, NCH - 1);
    // Offsets in `h` of the rows xl..=xl + 16 (see HIST_ROWS).
    let mut rows = [0usize; 17];
    let mut r = xl % HIST_ROWS;
    for row in rows.iter_mut() {
        *row = r * 64;
        r = if r == HIST_ROWS - 1 { 0 } else { r + 1 };
    }
    let (r14, r15, r16) = (rows[14], rows[15], rows[16]);

    h[r15 + 60] = g[xl + 18 * 16];
    h[r15 + 62] = g[xl];
    h[r16 + 60] = g[xl + 1 + 18 * 16];
    h[r16 + 62] = g[xl + 1];
    if stereo {
        h[r15 + 61] = g[xr + 18 * 16];
        h[r15 + 63] = g[xr];
        h[r16 + 61] = g[xr + 1 + 18 * 16];
        h[r16 + 63] = g[xr + 1];
        synth_pair(out, dr, NCH, h, &rows, 0, 1);
        synth_pair(out, dr + 32 * NCH, NCH, h, &rows, 1, 1);
    }
    synth_pair(out, dl, NCH, h, &rows, 0, 0);
    synth_pair(out, dl + 32 * NCH, NCH, h, &rows, 1, 0);

    // g indices of the values copied into the history for position i.
    let mut hi = 18 * (31 - 14); // 18 * (31 - i)
    let mut lo = 18 * (1 + 14); // 18 * (1 + i)
    for i in (0..15).rev() {
        let q = 4 * i;
        h[r15 + q] = g[xl + hi];
        h[r15 + q + 2] = g[xl + 1 + hi];
        h[r16 + q] = g[xl + 1 + lo];
        h[r14 + q + 2] = g[xl + lo];
        if stereo {
            h[r15 + q + 1] = g[xr + hi];
            h[r15 + q + 3] = g[xr + 1 + hi];
            h[r16 + q + 1] = g[xr + 1 + lo];
            h[r14 + q + 3] = g[xr + lo];
        }
        hi += 18;
        lo -= 18;

        let w: &[f32; 16] = G_WIN[16 * (14 - i)..16 * (14 - i) + 16].try_into().unwrap();
        let ([a0, a1, a2, a3], [b0, b1, b2, b3]) = if stereo {
            accumulate4(h, &rows, w, q)
        } else {
            let (a0, b0, a2, b2) = accumulate(h, &rows, w, q);
            ([a0, 0.0, a2, 0.0], [b0, 0.0, b2, 0.0])
        };

        if stereo {
            out[dr + (15 - i) * NCH] = S::from_synth(a1);
            out[dr + (17 + i) * NCH] = S::from_synth(b1);
        }
        out[dl + (15 - i) * NCH] = S::from_synth(a0);
        out[dl + (17 + i) * NCH] = S::from_synth(b0);
        if stereo {
            out[dr + (47 - i) * NCH] = S::from_synth(a3);
            out[dr + (49 + i) * NCH] = S::from_synth(b3);
        }
        out[dl + (47 - i) * NCH] = S::from_synth(a2);
        out[dl + (49 + i) * NCH] = S::from_synth(b2);
    }
}

/// Synthesizes one granule (576 samples per channel) from `g` (576 subband
/// samples per channel) into `pcm`.
///
/// A channel that isn't `active` is all zeros, and so is its DCT-II.
pub(crate) fn synth_granule<S: Sample>(
    h: &mut Hist,
    g: &mut [f32; 1152],
    nch: usize,
    active: [bool; 2],
    pcm: &mut [S],
) {
    crate::timed!(6, {
        for i in 0..nch {
            if active[i] {
                dct_ii((&mut g[576 * i..576 * i + 576]).try_into().unwrap(), 18);
            }
        }
    });
    crate::timed!(7, {
        for xl in (0..18).step_by(2) {
            if nch == 1 {
                synth::<S, 1>(g, xl, &mut pcm[32 * xl..32 * xl + 64], h);
            } else {
                synth::<S, 2>(g, xl, &mut pcm[64 * xl..64 * xl + 128], h);
            }
        }
    });
}

#[cfg(feature = "profile")]
pub(crate) fn bench_synth_only(h: &mut Hist, g: &[f32; 1152], pcm: &mut [i16; 1152]) {
    for xl in (0..18).step_by(2) {
        synth::<i16, 1>(g, xl, &mut pcm[32 * xl..32 * xl + 64], h);
    }
}

#[cfg(feature = "profile")]
pub(crate) fn bench_dct_ii(g: &mut [f32; 1152]) {
    dct_ii((&mut g[..576]).try_into().unwrap(), 18);
}
