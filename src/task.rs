use core::intrinsics::transmute;
use uart;
use main;

const STACK_BYTES: usize = 1024;
// TODO: use 4 or 8 depending on architecture.
const STACK_SIZE: usize = STACK_BYTES / 4;

pub struct TaskDescriptor {
    pub tid: u32,
    stack_ptr: *const usize,
    stack: [usize; STACK_SIZE],
}

// All tasks start with this stub. It enforces calling exit when the task returns. This function
// always runs in usermode.
extern fn task_stub() {
    loop {
        uart::putstr(b"U\n");
        uart::putptr(main::get_cpsr() as *const ());
        unsafe { asm!("swi 0" :::: "volatile") };
    }
}

extern {
    fn kernel_exit(stack_ptr: *const usize);
}

impl TaskDescriptor {
    pub fn new(tid: u32) -> TaskDescriptor {
        return TaskDescriptor {
            tid: tid,
            stack_ptr: 0 as *const _,
            stack: [0; STACK_SIZE],
        }
    }

    pub fn reset_stack(self: &mut TaskDescriptor) {
        let stack_ptr = unsafe { (&self.stack as *const usize).offset(STACK_SIZE as isize) };
        self.stack_ptr = stack_ptr;

        let mut counter = STACK_SIZE;
        let mut push = |x: usize| {
            counter -= 1;
            self.stack[counter] = x;
        };

        // Note that the task's state is stored above the stack, in the location which will be
        // written to next according the the ABI. However, we do not bother decrementing the
        // stack pointer.
        // Also, the initially unused registers are numbered. This makes it easier to tell if
        // the registers have been misaligned.
        push(unsafe { transmute(task_stub as *const ()) }); // r15/pc
        push(0); // r14/lr
        push(stack_ptr as usize); // r13/sp
        push(12); // r12/ip
        push(11); // r11/fp
        push(10); // r10/sl
        push(9); // r9, syscall number
        push(8); // r8
        push(7); // r7
        push(6); // r6
        push(5); // r5
        push(4); // r4
        push(3); // r3
        push(2); // r2
        push(1); // r1
        push(0x10); // cpsr TODO: totally insecure
    }

    // TODO: Actually, this very much mutates TaskDescriptor's stack.
    pub fn enter(self: &TaskDescriptor) {
        unsafe {
            // TODO: need to read stack back.
            kernel_exit(self.stack_ptr)
        };
    }
}
