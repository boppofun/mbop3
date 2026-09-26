//! Lockstep comparison of two decoders over an input.

use crate::decoders::{Decoder, Info, MAX_SAMPLES_PER_FRAME, Sample};

/// How input bytes are fed to a decoder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Driver {
    /// The whole file in one buffer, advancing by `frame_bytes` (like `mp3dec_load_buf`).
    Slice,
    /// A fixed size window that is topped up from the file before every call
    /// (how awedio's Mp3Decoder drives rmp3, with a 2048 byte window).
    Window(usize),
}

impl std::fmt::Display for Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Driver::Slice => write!(f, "slice"),
            Driver::Window(n) => write!(f, "window{n}"),
        }
    }
}

/// Feeds a decoder input according to a [Driver].
pub struct Feeder<'a> {
    data: &'a [u8],
    driver: Driver,
    /// Slice: position of the next unread byte. Window: position of the next
    /// byte to copy into `window`.
    pos: usize,
    window: Vec<u8>,
    /// Absolute input offset of the data passed to the last decode call.
    pub offset: usize,
}

impl<'a> Feeder<'a> {
    pub fn new(data: &'a [u8], driver: Driver) -> Self {
        let window = match driver {
            Driver::Slice => Vec::new(),
            Driver::Window(n) => Vec::with_capacity(n),
        };
        Feeder {
            data,
            driver,
            pos: 0,
            window,
            offset: 0,
        }
    }

    /// Decodes the next frame. Returns None at the end of the input.
    pub fn next<S: Sample, D: Decoder<S>>(
        &mut self,
        dec: &mut D,
        pcm: &mut [S; MAX_SAMPLES_PER_FRAME],
    ) -> Option<(usize, Info)> {
        {
            let input: &[u8] = match self.driver {
                Driver::Slice => &self.data[self.pos..],
                Driver::Window(cap) => {
                    let take = (cap - self.window.len()).min(self.data.len() - self.pos);
                    self.window
                        .extend_from_slice(&self.data[self.pos..self.pos + take]);
                    self.pos += take;
                    &self.window
                }
            };
            if input.is_empty() {
                return None;
            }
            self.offset = match self.driver {
                Driver::Slice => self.pos,
                Driver::Window(_) => self.pos - self.window.len(),
            };
            let (samples, info) = dec.decode(input, Some(pcm));
            let consumed = info.frame_bytes as usize;
            if consumed == 0 {
                return None;
            }
            match self.driver {
                Driver::Slice => self.pos += consumed,
                Driver::Window(_) => {
                    self.window.drain(..consumed);
                }
            }
            Some((samples, info))
        }
    }
}

/// Pass/fail criteria for comparing PCM.
#[derive(Debug, Clone, Copy)]
pub enum Criteria {
    /// Every sample must have the same bits.
    Exact,
    /// Per-file limits on the difference, in units of full scale (+-1.0).
    /// Frame info must still match exactly.
    Tolerance { max_abs: f64, max_rms: f64 },
}

impl Criteria {
    /// ISO/IEC 11172-4 "full accuracy" limits: RMS < 2^-15/sqrt(12), every sample within 2^-14.
    pub fn iso_full_accuracy() -> Self {
        Criteria::Tolerance {
            max_abs: 2f64.powi(-14),
            max_rms: 2f64.powi(-15) / 12f64.sqrt(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Stats {
    pub frames: u64,
    pub samples: u64,
    pub differing_samples: u64,
    pub sum_sq_diff: f64,
    pub max_abs_diff: f64,
}

impl Stats {
    pub fn rms(&self) -> f64 {
        if self.samples == 0 {
            0.0
        } else {
            (self.sum_sq_diff / self.samples as f64).sqrt()
        }
    }
}

#[derive(Debug)]
pub struct Mismatch {
    pub driver: Driver,
    pub frame: u64,
    pub offset: usize,
    pub what: String,
}

pub fn compare<S: Sample, A: Decoder<S>, B: Decoder<S>>(
    data: &[u8],
    driver: Driver,
    criteria: Criteria,
) -> (Stats, Option<Mismatch>) {
    let mut a = A::new();
    let mut b = B::new();
    let mut fa = Feeder::new(data, driver);
    let mut fb = Feeder::new(data, driver);
    let mut pa = [S::default(); MAX_SAMPLES_PER_FRAME];
    let mut pb = [S::default(); MAX_SAMPLES_PER_FRAME];
    let mut stats = Stats::default();
    let mismatch = |frame: u64, offset: usize, what: String| Mismatch {
        driver,
        frame,
        offset,
        what,
    };
    loop {
        let ra = fa.next(&mut a, &mut pa);
        let rb = fb.next(&mut b, &mut pb);
        let n = match (ra, rb) {
            (None, None) => break,
            (Some((sa, ia)), Some((sb, ib))) => {
                if sa != sb || ia != ib || fa.offset != fb.offset {
                    let what = format!(
                        "frame differs: {} samples={sa} {ia:?} @{} vs {} samples={sb} {ib:?} @{}",
                        A::name(),
                        fa.offset,
                        B::name(),
                        fb.offset
                    );
                    return (stats, Some(mismatch(stats.frames, fa.offset, what)));
                }
                sa * ia.channels.max(1) as usize
            }
            (ra, rb) => {
                let what = format!(
                    "stream length differs: {} {:?} vs {} {:?}",
                    A::name(),
                    ra,
                    B::name(),
                    rb
                );
                return (
                    stats,
                    Some(mismatch(stats.frames, fa.offset.max(fb.offset), what)),
                );
            }
        };
        for i in 0..n {
            let (x, y) = (pa[i], pb[i]);
            if x.bits() != y.bits() {
                stats.differing_samples += 1;
                let d = (x.to_unit() - y.to_unit()).abs();
                let d = if d.is_nan() { f64::INFINITY } else { d };
                stats.sum_sq_diff += d * d;
                stats.max_abs_diff = stats.max_abs_diff.max(d);
                if let Criteria::Exact = criteria {
                    let what = format!(
                        "sample {i} differs: {} {x:?} vs {} {y:?}",
                        A::name(),
                        B::name()
                    );
                    return (stats, Some(mismatch(stats.frames, fa.offset, what)));
                }
            }
        }
        stats.samples += n as u64;
        stats.frames += 1;
    }
    if let Criteria::Tolerance { max_abs, max_rms } = criteria
        && (stats.max_abs_diff > max_abs || stats.rms() > max_rms)
    {
        let what = format!(
            "outside tolerance: max_abs {:.3e} (limit {max_abs:.3e}) rms {:.3e} (limit {max_rms:.3e})",
            stats.max_abs_diff,
            stats.rms()
        );
        return (stats, Some(mismatch(stats.frames, 0, what)));
    }
    (stats, None)
}

/// Decodes the whole input with the slice driver and returns interleaved PCM.
pub fn decode_all<S: Sample, D: Decoder<S>>(data: &[u8]) -> (Vec<S>, Option<Info>) {
    let mut d = D::new();
    let mut f = Feeder::new(data, Driver::Slice);
    let mut pcm = [S::default(); MAX_SAMPLES_PER_FRAME];
    let mut out = Vec::new();
    let mut last = None;
    while let Some((samples, info)) = f.next(&mut d, &mut pcm) {
        if samples > 0 {
            out.extend_from_slice(&pcm[..samples * info.channels as usize]);
            last = Some(info);
        }
    }
    (out, last)
}
