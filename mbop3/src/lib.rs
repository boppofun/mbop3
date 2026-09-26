//! mbop3: an MP3 (MPEG-1/2/2.5 Layer III) decoder derived from
//! [minimp3](https://github.com/lieff/minimp3).
#![no_std]

mod minimp3;

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

/// MP3 decoder state.
pub struct Decoder(minimp3::mp3dec_t);

impl Decoder {
    pub fn new() -> Self {
        // SAFETY: mp3dec_t is plain old data, all zeros is valid.
        let mut dec = Decoder(unsafe { core::mem::zeroed() });
        unsafe { minimp3::mp3dec_init(&mut dec.0) };
        dec
    }

    /// Finds and decodes the next frame in `mp3`, returning the number of
    /// samples per channel written to `pcm` and info about the frame.
    ///
    /// With `pcm` None the frame is parsed but not decoded.
    pub fn decode_frame(
        &mut self,
        mp3: &[u8],
        pcm: Option<&mut [i16; MAX_SAMPLES_PER_FRAME]>,
    ) -> (usize, FrameInfo) {
        let mut info = minimp3::mp3dec_frame_info_t {
            frame_bytes: 0,
            frame_offset: 0,
            channels: 0,
            hz: 0,
            layer: 0,
            bitrate_kbps: 0,
        };
        let pcm = pcm.map_or(core::ptr::null_mut(), |p| p.as_mut_ptr());
        let len = i32::try_from(mp3.len()).unwrap_or(i32::MAX);
        let samples =
            unsafe { minimp3::mp3dec_decode_frame(&mut self.0, mp3.as_ptr(), len, pcm, &mut info) };
        let info = FrameInfo {
            frame_bytes: info.frame_bytes,
            frame_offset: info.frame_offset,
            channels: info.channels,
            hz: info.hz,
            layer: info.layer,
            bitrate_kbps: info.bitrate_kbps,
        };
        (samples as usize, info)
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
