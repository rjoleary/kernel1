pub fn halt() -> ! {
    loop {
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { asm!("" :::: "volatile") }
    }
}
