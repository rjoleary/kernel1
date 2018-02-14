use scheduler::Scheduler;
use uart;

pub fn get_cpsr() -> usize {
    let cpsr: usize;
    unsafe {
        asm!("mrs $0, cpsr"
             : "=r"(cpsr));
    }
    cpsr
}

pub fn main_loop<'a>(scheduler: &'a Scheduler<'a>) -> ! {
    loop {
        uart::putstr(b"K\n");
        uart::putptr(get_cpsr() as *const ());
        let current = scheduler.next();
        current.enter();
    }
}
