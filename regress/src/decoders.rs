//! A common interface over mbop3 and the C reference so the harness can drive
//! both identically.

use mbop3_reference as reference;

pub const MAX_SAMPLES_PER_FRAME: usize = mbop3::MAX_SAMPLES_PER_FRAME;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Info {
    pub frame_bytes: i32,
    pub frame_offset: i32,
    pub channels: i32,
    pub hz: i32,
    pub layer: i32,
    pub bitrate_kbps: i32,
}

pub trait Sample: Copy + Default + PartialEq + std::fmt::Debug + Send + 'static {
    /// Bit pattern used for exact comparison (so e.g. -0.0 != 0.0 and NaN == NaN).
    fn bits(self) -> u32;
    /// Value scaled so full scale is +-1.0.
    fn to_unit(self) -> f64;
}

impl Sample for i16 {
    fn bits(self) -> u32 {
        self as u16 as u32
    }
    fn to_unit(self) -> f64 {
        self as f64 / 32768.0
    }
}

impl Sample for f32 {
    fn bits(self) -> u32 {
        self.to_bits()
    }
    fn to_unit(self) -> f64 {
        self as f64
    }
}

pub trait Decoder<S: Sample> {
    fn name() -> &'static str;
    fn new() -> Self;
    /// Returns samples per channel.
    fn decode(&mut self, mp3: &[u8], pcm: Option<&mut [S; MAX_SAMPLES_PER_FRAME]>) -> (usize, Info);
}

pub struct Mbop3(Box<mbop3::Decoder>);

impl Decoder<i16> for Mbop3 {
    fn name() -> &'static str {
        "mbop3"
    }
    fn new() -> Self {
        Mbop3(Box::default())
    }
    fn decode(
        &mut self,
        mp3: &[u8],
        pcm: Option<&mut [i16; MAX_SAMPLES_PER_FRAME]>,
    ) -> (usize, Info) {
        let (samples, i) = self.0.decode_frame(mp3, pcm);
        (
            samples,
            Info {
                frame_bytes: i.frame_bytes,
                frame_offset: i.frame_offset,
                channels: i.channels,
                hz: i.hz,
                layer: i.layer,
                bitrate_kbps: i.bitrate_kbps,
            },
        )
    }
}

fn from_ref(i: reference::FrameInfo) -> Info {
    Info {
        frame_bytes: i.frame_bytes,
        frame_offset: i.frame_offset,
        channels: i.channels,
        hz: i.hz,
        layer: i.layer,
        bitrate_kbps: i.bitrate_kbps,
    }
}

pub struct RefI16(reference::i16::Decoder);

impl Decoder<i16> for RefI16 {
    fn name() -> &'static str {
        "minimp3"
    }
    fn new() -> Self {
        RefI16(reference::i16::Decoder::new())
    }
    fn decode(
        &mut self,
        mp3: &[u8],
        pcm: Option<&mut [i16; MAX_SAMPLES_PER_FRAME]>,
    ) -> (usize, Info) {
        let (s, i) = self.0.decode_frame(mp3, pcm);
        (s, from_ref(i))
    }
}

pub struct RefF32(reference::f32::Decoder);

impl Decoder<f32> for RefF32 {
    fn name() -> &'static str {
        "minimp3-f32"
    }
    fn new() -> Self {
        RefF32(reference::f32::Decoder::new())
    }
    fn decode(
        &mut self,
        mp3: &[u8],
        pcm: Option<&mut [f32; MAX_SAMPLES_PER_FRAME]>,
    ) -> (usize, Info) {
        let (s, i) = self.0.decode_frame(mp3, pcm);
        (s, from_ref(i))
    }
}
