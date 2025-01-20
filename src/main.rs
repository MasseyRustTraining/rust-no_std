#![no_std]
#![no_main]

use heapless::String;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    unsafe { libc::abort() }
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut s: String<16> = String::new();
    s.push_str("hello\n").unwrap();
    let result = unsafe { libc::write(1, s.as_ptr() as *const libc::c_void, s.len()) };
    if result != s.len() as isize {
        return 1;
    }
    0
}
