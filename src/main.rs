use scheduler::Scheduler;
use uart;

pub fn inner<'a>(scheduler: &'a Scheduler<'a>) {
}

pub fn main_loop<'a>(scheduler: &'a Scheduler<'a>) -> ! {
    loop {
        let next_td = scheduler.next();
        uart::putc(b'0' + next_td.tid as u8);
    }
}
