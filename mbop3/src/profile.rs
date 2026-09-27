//! Optional per-stage cycle counting (feature "profile", development only), for
//! tuning on a device. This is the only unsafe code, and only with this feature.
//!
//! Call [`set_clock`] with a cycle counter, decode, then read [`take`].

use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering::Relaxed};

/// Stages timed, in the order [`take`] returns them.
pub const STAGES: [&str; 8] = [
    "side_info",
    "scalefactors",
    "huffman",
    "stereo",
    "reorder_antialias",
    "imdct",
    "dct_ii",
    "synth",
];

static CLOCK: AtomicUsize = AtomicUsize::new(0);
static TOTALS: [AtomicU32; 8] = [const { AtomicU32::new(0) }; 8];

/// Sets the cycle counter used for timing.
pub fn set_clock(clock: fn() -> u32) {
    CLOCK.store(clock as usize, Relaxed);
}

/// Returns and resets the cycles spent in each of [`STAGES`].
pub fn take() -> [u32; 8] {
    core::array::from_fn(|i| TOTALS[i].swap(0, Relaxed))
}

#[inline(always)]
pub(crate) fn now() -> u32 {
    let c = CLOCK.load(Relaxed);
    if c == 0 {
        return 0;
    }
    // SAFETY: CLOCK only ever holds 0 or a `fn() -> u32` stored by set_clock.
    #[allow(unsafe_code)]
    let f: fn() -> u32 = unsafe { core::mem::transmute::<usize, fn() -> u32>(c) };
    f()
}

#[inline(always)]
pub(crate) fn add(stage: usize, start: u32) {
    TOTALS[stage].fetch_add(now().wrapping_sub(start), Relaxed);
}

/// Hot-cache micro benchmarks of the main kernels: cycles per granule
/// (mono) for [synth, dct_ii, imdct36], averaged over `iters` runs.
#[doc(hidden)]
pub fn bench_kernels(iters: u32) -> [u32; 3] {
    let mut hist = [0f32; crate::synth::HIST_ROWS * 64];
    let mut g = [0f32; 1152];
    let mut seed = 12345u32;
    for v in g.iter_mut() {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        *v = ((seed >> 16) as f32 - 32768.0) * 0.01;
    }
    let mut pcm = [0i16; 1152];
    let mut overlap = [0f32; 288];
    let mut out = [0u32; 3];
    let start = now();
    for _ in 0..iters {
        crate::synth::bench_synth_only(&mut hist, &g, &mut pcm);
    }
    out[0] = now().wrapping_sub(start) / iters;
    let start = now();
    for _ in 0..iters {
        crate::synth::bench_dct_ii(&mut g);
    }
    out[1] = now().wrapping_sub(start) / iters;
    let start = now();
    for _ in 0..iters {
        crate::layer3::imdct_gr(&mut g[..576], &mut overlap, 0, 0, 32);
    }
    out[2] = now().wrapping_sub(start) / iters;
    core::hint::black_box((&pcm, &g));
    out
}

/// FPU micro benchmark: cycles per operation for [dependent mul+add chain,
/// 4 independent chains, dependent add chain, f32 load+add from an array].
#[doc(hidden)]
pub fn bench_fpu() -> [u32; 4] {
    use core::hint::black_box;
    const N: u32 = 1000;
    let (a, b) = (black_box(0.999f32), black_box(0.001f32));
    let mut out = [0u32; 4];

    let mut x = black_box(1.0f32);
    let t = now();
    for _ in 0..N {
        x = x * a + b;
    }
    out[0] = now().wrapping_sub(t) / N;
    black_box(x);

    let (mut x0, mut x1, mut x2, mut x3) = black_box((1.0f32, 2.0f32, 3.0f32, 4.0f32));
    let t = now();
    for _ in 0..N {
        x0 = x0 * a + b;
        x1 = x1 * a + b;
        x2 = x2 * a + b;
        x3 = x3 * a + b;
    }
    out[1] = now().wrapping_sub(t) / (4 * N);
    black_box((x0, x1, x2, x3));

    let mut x = black_box(1.0f32);
    let t = now();
    for _ in 0..N {
        x += b;
    }
    out[2] = now().wrapping_sub(t) / N;
    black_box(x);

    let arr = black_box([0.5f32; 64]);
    let mut x = 0.0f32;
    let t = now();
    for i in 0..N as usize {
        x += arr[i & 63];
    }
    out[3] = now().wrapping_sub(t) / N;
    black_box(x);
    out
}
