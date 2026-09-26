//! Measures peak stack use of a closure by painting the stack below the current
//! frame, running the closure, and finding the lowest overwritten byte.
//!
//! Host (x86_64) numbers are a proxy for the device: frame sizes differ per
//! architecture, but big items like scratch buffers show up the same.

const PATTERN: u8 = 0xA5;
/// How far below the measuring frame to paint. Must be well within the thread's stack.
const PAINT_BYTES: usize = 256 * 1024;
/// Gap left unpainted directly below the measuring frame for `paint`'s own frame.
const GAP: usize = 512;
const THREAD_STACK: usize = 1024 * 1024;

#[inline(never)]
fn stack_ptr() -> usize {
    let x = 0u8;
    std::hint::black_box(&x) as *const u8 as usize
}

#[inline(never)]
fn paint(top: usize) {
    let low = top - PAINT_BYTES;
    for a in low..top - GAP {
        // SAFETY: the region is inside this thread's (1 MiB) stack and below
        // any live frame.
        unsafe { std::ptr::write_volatile(a as *mut u8, PATTERN) };
    }
}

#[inline(never)]
fn lowest_dirty(top: usize) -> usize {
    let low = top - PAINT_BYTES;
    for a in low..top - GAP {
        // SAFETY: see paint.
        if unsafe { std::ptr::read_volatile(a as *const u8) } != PATTERN {
            return a;
        }
    }
    top - GAP
}

#[inline(never)]
fn measure_here<F: FnOnce()>(f: F) -> usize {
    let top = stack_ptr();
    paint(top);
    f();
    top - lowest_dirty(top)
}

/// Peak stack bytes used by `f`, net of the measurement overhead.
pub fn measure<F: FnOnce() + Send>(f: F) -> usize {
    std::thread::scope(|s| {
        std::thread::Builder::new()
            .stack_size(THREAD_STACK)
            .spawn_scoped(s, || {
                let baseline = measure_here(|| {});
                measure_here(f).saturating_sub(baseline)
            })
            .unwrap()
            .join()
            .unwrap()
    })
}
