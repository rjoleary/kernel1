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

extern {
    fn setup_swi();

    // Markers in the linker script
    fn _DataStart();
    fn _DataEnd();
    fn _BssStart();
    fn _BssEnd();
    fn _TextStart();
    fn _TextEnd();
}

#[start]
#[no_mangle]
pub fn start(_argc: isize, _argv: *const *const u8) -> isize {
    uart::putstr(b"Start\r\n");

    uart::putptr(_DataStart as *const ());
    uart::putptr(_DataEnd as *const ());
    uart::putptr(_BssStart as *const ());
    uart::putptr(_BssEnd as *const ());
    uart::putptr(_TextStart as *const ());
    uart::putptr(_TextEnd as *const ());

    // Set software interrupt.
    unsafe { setup_swi() };

    // Setup init task.
    let mut init = task::TaskDescriptor::new(1);
    init.reset_stack();
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
