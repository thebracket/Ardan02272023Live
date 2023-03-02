#![no_std]
#![no_main]

use libc::printf;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // notice that we have to zero terminate the string in Rust
    unsafe { printf(b"Hello world!\n\0".as_ptr() as *const _) };
    0
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}