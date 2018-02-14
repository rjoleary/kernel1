use core::intrinsics;

pub fn putc(data: u8) {
    // UART address is specific to QEMU's virt machine.
    let uart_rx = 0x09000000 as *mut u32;
    unsafe {
        intrinsics::volatile_store(uart_rx, data as u32);
    }
}
