//! mbop3: a small `no_std` MP3 (MPEG-1/2/2.5 Layer III) decoder, derived from
//! [minimp3](https://github.com/lieff/minimp3).
//!
//! ```no_run
//! let mp3: &[u8] = &[]; // at least several frames of input
//! let mut decoder = mbop3::Decoder::new();
//! let mut pcm = [0i16; mbop3::MAX_SAMPLES_PER_FRAME];
//! let (samples, info) = decoder.decode_frame(mp3, Some(&mut pcm));
//! // pcm[..samples * info.channels as usize] is interleaved audio.
//! // Skip info.frame_bytes of input before the next call.
//! ```
#![no_std]
#![forbid(unsafe_code)]
// Constants are kept textually identical to minimp3's, and loops mirror its
// indexing, which keeps the output bit-exact and the code easy to compare.
#![allow(
    clippy::excessive_precision,
    clippy::approx_constant,
    clippy::needless_range_loop
)]

mod bits;
mod decoder;
mod header;
mod layer3;
mod synth;
mod tables;

pub use decoder::Decoder;

/// Maximum samples (all channels) a single frame can produce.
pub const MAX_SAMPLES_PER_FRAME: usize = 1152 * 2;

/// Information about the frame found by [`Decoder::decode_frame`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrameInfo {
    /// Bytes of input consumed, including any skipped data before the frame.
    /// 0 means no frame was found and more input is needed.
    pub frame_bytes: i32,
    /// Offset of the frame within the input.
    pub frame_offset: i32,
    pub channels: i32,
    pub hz: i32,
    pub layer: i32,
    pub bitrate_kbps: i32,
}

/// An output sample type: `i16`, or `f32` in the range -1.0..1.0.
pub trait Sample: Copy + Default + private::Sealed {
    #[doc(hidden)]
    fn from_synth(sample: f32) -> Self;
}

impl Sample for i16 {
    #[inline]
    fn from_synth(sample: f32) -> i16 {
        if sample >= 32766.5 {
            return 32767;
        }
        if sample <= -32767.5 {
            return -32768;
        }
        let s = (sample + 0.5) as i16;
        s - (s < 0) as i16 // away from zero, to be compliant
    }
}

impl Sample for f32 {
    #[inline]
    fn from_synth(sample: f32) -> f32 {
        sample * (1.0 / 32768.0)
    }
}

mod private {
    pub trait Sealed {}
    impl Sealed for i16 {}
    impl Sealed for f32 {}
}
