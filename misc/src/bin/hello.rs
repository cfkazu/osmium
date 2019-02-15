#![no_std]
#![no_main]
#![feature(asm)]
extern crate misc;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let buf = "Hello Syscall";

    misc::sys_write(buf.as_bytes(), buf.len());

    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}
