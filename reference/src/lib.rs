//! The original C minimp3 decoder, used as the golden reference for mbop3.
//!
//! Built from a pinned minimp3 commit (see `minimp3/COMMIT`) with
//! `MINIMP3_ONLY_MP3` and `MINIMP3_NO_SIMD`, in both int16 and float output
//! variants.

use std::ffi::{c_int, c_void};

/// Maximum samples (all channels) a single frame can produce.
pub const MAX_SAMPLES_PER_FRAME: usize = 1152 * 2;

/// Mirrors minimp3's `mp3dec_frame_info_t`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrameInfo {
    pub frame_bytes: i32,
    pub frame_offset: i32,
    pub channels: i32,
    pub hz: i32,
    pub layer: i32,
    pub bitrate_kbps: i32,
}

macro_rules! variant {
    ($mod:ident, $sample:ty, $new:ident, $free:ident, $decode:ident, $dec_size:ident, $scratch_size:ident) => {
        unsafe extern "C" {
            fn $new() -> *mut c_void;
            fn $free(dec: *mut c_void);
            fn $decode(
                dec: *mut c_void,
                mp3: *const u8,
                mp3_bytes: c_int,
                pcm: *mut c_void,
                info: *mut c_int,
            ) -> c_int;
            fn $dec_size() -> usize;
            fn $scratch_size() -> usize;
        }

        pub mod $mod {
            use super::*;

            /// A C minimp3 decoder instance.
            pub struct Decoder(*mut c_void);

            // The C decoder has no thread affinity.
            unsafe impl Send for Decoder {}

            impl Decoder {
                pub fn new() -> Self {
                    let ptr = unsafe { $new() };
                    assert!(!ptr.is_null());
                    Decoder(ptr)
                }

                /// `mp3dec_decode_frame`. Returns samples per channel and the frame info.
                /// With `pcm` None, the frame is parsed but not decoded.
                pub fn decode_frame(
                    &mut self,
                    mp3: &[u8],
                    pcm: Option<&mut [$sample; MAX_SAMPLES_PER_FRAME]>,
                ) -> (usize, FrameInfo) {
                    let mut info = [0 as c_int; 6];
                    let pcm = pcm.map_or(std::ptr::null_mut(), |p| p.as_mut_ptr() as *mut c_void);
                    let len = c_int::try_from(mp3.len()).expect("input fits in c_int");
                    let samples =
                        unsafe { $decode(self.0, mp3.as_ptr(), len, pcm, info.as_mut_ptr()) };
                    let info = FrameInfo {
                        frame_bytes: info[0],
                        frame_offset: info[1],
                        channels: info[2],
                        hz: info[3],
                        layer: info[4],
                        bitrate_kbps: info[5],
                    };
                    (samples as usize, info)
                }

                /// `sizeof(mp3dec_t)`
                pub fn decoder_size() -> usize {
                    unsafe { $dec_size() }
                }

                /// `sizeof(mp3dec_scratch_t)`: this lives on the stack for every decode call.
                pub fn scratch_size() -> usize {
                    unsafe { $scratch_size() }
                }
            }

            impl Default for Decoder {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl Drop for Decoder {
                fn drop(&mut self) {
                    unsafe { $free(self.0) }
                }
            }
        }
    };
}

variant!(
    i16,
    i16,
    mbop3ref_i16_new,
    mbop3ref_i16_free,
    mbop3ref_i16_decode,
    mbop3ref_i16_decoder_size,
    mbop3ref_i16_scratch_size
);
variant!(
    f32,
    f32,
    mbop3ref_f32_new,
    mbop3ref_f32_free,
    mbop3ref_f32_decode,
    mbop3ref_f32_decoder_size,
    mbop3ref_f32_scratch_size
);
