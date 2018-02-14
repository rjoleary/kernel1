#![feature(core_intrinsics)]
#![feature(lang_items, start)]
#![feature(asm)]
#![no_std]
#![no_main]
use core::intrinsics;

mod uart;
mod task;
mod scheduler;
mod main;
mod halt;

#[start]
#[no_mangle]
pub fn start(_argc: isize, _argv: *const *const u8) -> isize {
    for &x in b"start\r\n" {
        uart::putc(x);
    }

    // Setup init task.
    let init = task::TaskDescriptor{tid: 1};
    let scheduler = scheduler::Scheduler::new(&init);

    main::main_loop(&scheduler);
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
