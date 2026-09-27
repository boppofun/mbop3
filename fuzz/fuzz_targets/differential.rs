//! Decodes arbitrary input with mbop3 and C minimp3 in lockstep, the whole
//! input at once and through a small refilled window, and compares them.
#![no_main]

use libfuzzer_sys::fuzz_target;
use mbop3::MAX_SAMPLES_PER_FRAME;

fuzz_target!(|data: &[u8]| {
    // The first byte picks the window size of the second pass.
    let Some((&w, data)) = data.split_first() else { return };
    run(data, None);
    run(data, Some(64 + w as usize * 16));
});

fn run(data: &[u8], window: Option<usize>) {
    let mut ours = Box::new(mbop3::Decoder::new());
    let mut theirs = mbop3_reference::i16::Decoder::new();
    let mut pa = Box::new([0i16; MAX_SAMPLES_PER_FRAME]);
    let mut pb = Box::new([0i16; MAX_SAMPLES_PER_FRAME]);
    let mut pos = 0;
    while pos < data.len() {
        let end = window.map_or(data.len(), |w| (pos + w).min(data.len()));
        let input = &data[pos..end];
        let (sa, ia) = ours.decode_frame(input, Some(&mut pa));
        let (sb, ib) = theirs.decode_frame(input, Some(&mut pb));
        assert_eq!(
            (sa, ia.frame_bytes, ia.frame_offset, ia.channels, ia.hz, ia.layer, ia.bitrate_kbps),
            (sb, ib.frame_bytes, ib.frame_offset, ib.channels, ib.hz, ib.layer, ib.bitrate_kbps)
        );
        if cfg!(feature = "exact") {
            let n = sa * ia.channels as usize;
            assert!(pa[..n] == pb[..n], "pcm differs");
        }
        if ia.frame_bytes == 0 {
            break;
        }
        pos += ia.frame_bytes as usize;
    }
}
