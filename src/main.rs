#![feature(core_intrinsics)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
use core::intrinsics;

fn putc(data: u8) {
    // UART address is specific to QEMU's virt machine.
    let uart_rx = 0x09000000 as *mut u32;
    unsafe {
        intrinsics::volatile_store(uart_rx, data as u32);
    }
}

#[start]
#[no_mangle]
pub fn start(_argc: isize, _argv: *const *const u8) -> isize {
    for &i in b"0123456789" {
        putc(i);
    }
    0
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(_msg: core::fmt::Arguments,
                        _file: &'static str,
                        _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}
