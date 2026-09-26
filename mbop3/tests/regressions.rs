//! Inputs that once broke mbop3, decoded by mbop3 and C minimp3 side by side.
//! With the "exact" feature the output must be bit-identical; otherwise the
//! frame structure must match (these are corrupt streams).
//!
//! - mixed_block_8khz_reorder_overrun.mp3: a header bit flip makes an MPEG-2.5
//!   8 kHz mixed block frame, whose short block reorder runs past the channel's
//!   576 values (a panic before mbop3 reproduced minimp3's memory layout).

use mbop3::{Decoder, MAX_SAMPLES_PER_FRAME};

fn check(name: &str) {
    let data = std::fs::read(format!("{}/tests/data/{name}", env!("CARGO_MANIFEST_DIR"))).unwrap();
    let mut ours = Box::new(Decoder::new());
    let mut theirs = mbop3_reference::i16::Decoder::new();
    let (mut pa, mut pb) = ([0i16; MAX_SAMPLES_PER_FRAME], [0i16; MAX_SAMPLES_PER_FRAME]);
    let (mut a, mut b) = (&data[..], &data[..]);
    loop {
        let (sa, ia) = ours.decode_frame(a, Some(&mut pa));
        let (sb, ib) = theirs.decode_frame(b, Some(&mut pb));
        assert_eq!(
            (sa, ia.frame_bytes, ia.channels, ia.hz),
            (sb, ib.frame_bytes, ib.channels, ib.hz)
        );
        if cfg!(feature = "exact") {
            let n = sa * ia.channels as usize;
            assert_eq!(pa[..n], pb[..n]);
        }
        if ia.frame_bytes == 0 {
            break;
        }
        a = &a[ia.frame_bytes as usize..];
        b = &b[ib.frame_bytes as usize..];
    }
}

#[test]
fn mixed_block_8khz_reorder_overrun() {
    check("mixed_block_8khz_reorder_overrun.mp3");
}
