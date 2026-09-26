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
pub(crate) type Hist = [[f32; 64]; HIST_ROWS];

/// Physical history row of row `m` relative to band pair `xl`. Computed
/// where it is used so the compiler knows it is in bounds.
#[inline(always)]
fn row(xl: usize, m: usize) -> usize {
    (xl + m) % HIST_ROWS
}

/// Column `c` of the last group of 4 columns, over rows `m0..m0 + 15`.
#[inline(always)]
fn synth_pair<S: Sample>(
    out: &mut [S],
    p: usize,
    nch: usize,
    h: &Hist,
    xl: usize,
    m0: usize,
    c: usize,
) {
    let z = |m: usize, c: usize| h[row(xl, m0 + m)][60 + c];
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

/// Synthesizes 2 x 32 output samples per channel from subband sample rows
/// `xl` and `xl + 1` of `g` into `out` (64 samples per channel).
#[inline(always)]
fn synth<S: Sample, const NCH: usize>(g: &[f32; 1152], xl: usize, out: &mut [S], h: &mut Hist) {
    let stereo = NCH == 2;
    let xr = xl + 576 * (NCH - 1);
    let (dl, dr) = (0, NCH - 1);
    let (r14, r15, r16) = (row(xl, 14), row(xl, 15), row(xl, 16));

    h[r15][60] = g[xl + 18 * 16];
    h[r15][62] = g[xl];
    h[r16][60] = g[xl + 1 + 18 * 16];
    h[r16][62] = g[xl + 1];
    if stereo {
        h[r15][61] = g[xr + 18 * 16];
        h[r15][63] = g[xr];
        h[r16][61] = g[xr + 1 + 18 * 16];
        h[r16][63] = g[xr + 1];
        synth_pair(out, dr, NCH, h, xl, 0, 1);
        synth_pair(out, dr + 32 * NCH, NCH, h, xl, 1, 1);
    }
    synth_pair(out, dl, NCH, h, xl, 0, 0);
    synth_pair(out, dl + 32 * NCH, NCH, h, xl, 1, 0);

    for i in (0..15).rev() {
        let q = 4 * i;
        h[r15][q] = g[xl + 18 * (31 - i)];
        h[r15][q + 2] = g[xl + 1 + 18 * (31 - i)];
        h[r16][q] = g[xl + 1 + 18 * (1 + i)];
        h[r14][q + 2] = g[xl + 18 * (1 + i)];
        if stereo {
            h[r15][q + 1] = g[xr + 18 * (31 - i)];
            h[r15][q + 3] = g[xr + 1 + 18 * (31 - i)];
            h[r16][q + 1] = g[xr + 1 + 18 * (1 + i)];
            h[r14][q + 3] = g[xr + 18 * (1 + i)];
        }

        let mut a = [0f32; 4];
        let mut b = [0f32; 4];
        let w = &G_WIN[16 * (14 - i)..16 * (14 - i) + 16];
        // minimp3's S0 (k = 0), then S2 (odd k) and S1 (even k).
        for k in 0..8 {
            let (w0, w1) = (w[2 * k], w[2 * k + 1]);
            let vz: &[f32; 4] = h[row(xl, 15 - k)][q..q + 4].try_into().unwrap();
            let vy: &[f32; 4] = h[row(xl, k)][q..q + 4].try_into().unwrap();
            for j in (0..4).step_by(if stereo { 1 } else { 2 }) {
                let (z, y) = (vz[j], vy[j]);
                if k == 0 {
                    b[j] = z * w1 + y * w0;
                    a[j] = z * w0 - y * w1;
                } else if k % 2 == 1 {
                    b[j] += z * w1 + y * w0;
                    a[j] += y * w1 - z * w0;
                } else {
                    b[j] += z * w1 + y * w0;
                    a[j] += z * w0 - y * w1;
                }
            }
        }

        if stereo {
            out[dr + (15 - i) * NCH] = S::from_synth(a[1]);
            out[dr + (17 + i) * NCH] = S::from_synth(b[1]);
        }
        out[dl + (15 - i) * NCH] = S::from_synth(a[0]);
        out[dl + (17 + i) * NCH] = S::from_synth(b[0]);
        if stereo {
            out[dr + (47 - i) * NCH] = S::from_synth(a[3]);
            out[dr + (49 + i) * NCH] = S::from_synth(b[3]);
        }
        out[dl + (47 - i) * NCH] = S::from_synth(a[2]);
        out[dl + (49 + i) * NCH] = S::from_synth(b[2]);
    }
}

/// Synthesizes one granule (576 samples per channel) from `g` (576 subband
/// samples per channel) into `pcm`.
pub(crate) fn synth_granule<S: Sample>(
    h: &mut Hist,
    g: &mut [f32; 1152],
    nch: usize,
    pcm: &mut [S],
) {
    crate::timed!(6, {
        for i in 0..nch {
            dct_ii((&mut g[576 * i..576 * i + 576]).try_into().unwrap(), 18);
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
