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
