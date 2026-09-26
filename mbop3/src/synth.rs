//! Polyphase synthesis filterbank.

use crate::Sample;
use crate::tables::{G_SEC, G_WIN};

fn dct_ii(g: &mut [f32], n: usize) {
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

fn synth_pair<S: Sample>(pcm: &mut [S], p: usize, nch: usize, z: &[f32], mut zo: usize) {
    let mut a;
    a = (z[zo + 14 * 64] - z[zo]) * 29.0;
    a += (z[zo + 64] + z[zo + 13 * 64]) * 213.0;
    a += (z[zo + 12 * 64] - z[zo + 2 * 64]) * 459.0;
    a += (z[zo + 3 * 64] + z[zo + 11 * 64]) * 2037.0;
    a += (z[zo + 10 * 64] - z[zo + 4 * 64]) * 5153.0;
    a += (z[zo + 5 * 64] + z[zo + 9 * 64]) * 6574.0;
    a += (z[zo + 8 * 64] - z[zo + 6 * 64]) * 37489.0;
    a += z[zo + 7 * 64] * 75038.0;
    pcm[p] = S::from_synth(a);

    zo += 2;
    a = z[zo + 14 * 64] * 104.0;
    a += z[zo + 12 * 64] * 1567.0;
    a += z[zo + 10 * 64] * 9727.0;
    a += z[zo + 8 * 64] * 64019.0;
    a += z[zo + 6 * 64] * -9975.0;
    a += z[zo + 4 * 64] * -45.0;
    a += z[zo + 2 * 64] * 146.0;
    a += z[zo] * -5.0;
    pcm[p + 16 * nch] = S::from_synth(a);
}

/// Synthesizes 2 x 32 output samples per channel from subband sample rows
/// `xl` and `xl + 1` of `g`, writing them to `pcm[dl..]`. `lins` holds the
/// filterbank history, with this call's window starting at `lb`.
#[allow(clippy::too_many_arguments)]
fn synth<S: Sample>(
    g: &[f32],
    xl: usize,
    pcm: &mut [S],
    dl: usize,
    nch: usize,
    lins: &mut [f32],
    lb: usize,
) {
    let xr = xl + 576 * (nch - 1);
    let dr = dl + (nch - 1);
    let zlin = lb + 15 * 64;

    lins[zlin + 4 * 15] = g[xl + 18 * 16];
    lins[zlin + 4 * 15 + 1] = g[xr + 18 * 16];
    lins[zlin + 4 * 15 + 2] = g[xl];
    lins[zlin + 4 * 15 + 3] = g[xr];

    lins[zlin + 4 * 31] = g[xl + 1 + 18 * 16];
    lins[zlin + 4 * 31 + 1] = g[xr + 1 + 18 * 16];
    lins[zlin + 4 * 31 + 2] = g[xl + 1];
    lins[zlin + 4 * 31 + 3] = g[xr + 1];

    synth_pair(pcm, dr, nch, lins, lb + 4 * 15 + 1);
    synth_pair(pcm, dr + 32 * nch, nch, lins, lb + 4 * 15 + 64 + 1);
    synth_pair(pcm, dl, nch, lins, lb + 4 * 15);
    synth_pair(pcm, dl + 32 * nch, nch, lins, lb + 4 * 15 + 64);

    let mut w = 0;
    for i in (0..15).rev() {
        let mut a = [0f32; 4];
        let mut b = [0f32; 4];

        lins[zlin + 4 * i] = g[xl + 18 * (31 - i)];
        lins[zlin + 4 * i + 1] = g[xr + 18 * (31 - i)];
        lins[zlin + 4 * i + 2] = g[xl + 1 + 18 * (31 - i)];
        lins[zlin + 4 * i + 3] = g[xr + 1 + 18 * (31 - i)];
        lins[zlin + 4 * (i + 16)] = g[xl + 1 + 18 * (1 + i)];
        lins[zlin + 4 * (i + 16) + 1] = g[xr + 1 + 18 * (1 + i)];
        lins[zlin + 4 * i - 64 + 2] = g[xl + 18 * (1 + i)];
        lins[zlin + 4 * i - 64 + 3] = g[xr + 18 * (1 + i)];

        // k = 0..8 with minimp3's S0 (first), then alternating S2 (odd k) and S1 (even k).
        for k in 0..8 {
            let w0 = G_WIN[w];
            let w1 = G_WIN[w + 1];
            w += 2;
            let vz = zlin + 4 * i - k * 64;
            let vy = zlin + 4 * i - (15 - k) * 64;
            for j in 0..4 {
                let (z, y) = (lins[vz + j], lins[vy + j]);
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

        pcm[dr + (15 - i) * nch] = S::from_synth(a[1]);
        pcm[dr + (17 + i) * nch] = S::from_synth(b[1]);
        pcm[dl + (15 - i) * nch] = S::from_synth(a[0]);
        pcm[dl + (17 + i) * nch] = S::from_synth(b[0]);
        pcm[dr + (47 - i) * nch] = S::from_synth(a[3]);
        pcm[dr + (49 + i) * nch] = S::from_synth(b[3]);
        pcm[dl + (47 - i) * nch] = S::from_synth(a[2]);
        pcm[dl + (49 + i) * nch] = S::from_synth(b[2]);
    }
}

/// Synthesizes one granule (`nbands` * 32 samples per channel) from `g`
/// (576 subband samples per channel) into `pcm`.
pub(crate) fn synth_granule<S: Sample>(
    qmf_state: &mut [f32; 960],
    g: &mut [f32],
    nbands: usize,
    nch: usize,
    pcm: &mut [S],
    lins: &mut [f32],
) {
    for i in 0..nch {
        dct_ii(&mut g[576 * i..], nbands);
    }

    lins[..15 * 64].copy_from_slice(qmf_state);

    for i in (0..nbands).step_by(2) {
        synth(g, i, pcm, 32 * nch * i, nch, lins, i * 64);
    }
    if nch == 1 {
        for i in (0..15 * 64).step_by(2) {
            qmf_state[i] = lins[nbands * 64 + i];
        }
    } else {
        qmf_state.copy_from_slice(&lins[nbands * 64..nbands * 64 + 15 * 64]);
    }
}
