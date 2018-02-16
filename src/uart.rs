use core::intrinsics;

pub fn putc(data: u8) {
    // UART address is specific to QEMU's virt machine.
    // TODO: implement PL011 driver properly
    let uart_rx = 0x09000000 as *mut u32;
    //let uart_status = (0x09000000 + 0x018) as *mut u32;
    unsafe {
        //while intrinsics::volatile_load(uart_status) & (1<<7) == 0 {}
        intrinsics::volatile_store(uart_rx, data as u32);
    }
}

pub fn putstr(data: &[u8]) {
    for &c in data {
        if c != 0 {
            if c == b'\n' {
                putc(b'\r');
            }
            putc(c);
        }
    }
}

pub fn putptr<T>(data: *const T) {
    let data = data as usize;
    putstr(b"0x");
    for i in (0..8).rev() {
        putc(b"0123456789abcdef"[(data >> (i * 4)) % 16]);
    }
    putstr(b"\n");
}
