//! Exposes the decoder through one C symbol so LTO keeps exactly what a user needs.
#![no_std]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// # Safety
/// Pointers must be valid for the given lengths.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mbop3_sizecheck_decode(
    dec: *mut mbop3::Decoder,
    mp3: *const u8,
    len: usize,
    pcm: *mut [i16; mbop3::MAX_SAMPLES_PER_FRAME],
) -> usize {
    let (dec, mp3, pcm) = unsafe { (&mut *dec, core::slice::from_raw_parts(mp3, len), &mut *pcm) };
    dec.decode_frame(mp3, Some(pcm)).0
}

#[unsafe(no_mangle)]
pub extern "C" fn mbop3_sizecheck_bench_fpu() -> u32 {
    mbop3::profile::bench_fpu()[1]
}
