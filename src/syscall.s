@ vim:ft=armv5

.section .text.begin
.global entrypoint
.extern start
entrypoint:
    bl start

.global kernel_exit
.global setup_swi

kernel_entry:
    @ Switch to system mode to push user registers.
    msr cpsr_c, #0xdf
    stmfd sp, {r0-r15}
    @ r0: holds the user's stack pointer
    mov r0, sp
    @ Switch back to supervisor mode.
    msr cpsr, #0xd3
    @ Store lr as pc in user's stack.
    stmfd r0, {lr}
    @ Store the stored PSR in the user's stack.
    @ r1: holds the task's PSR
    mrs r1, spsr
    str r1, [r0, #-0x44]
    @ Restore kernel registers and return.
    ldmea sp, {r4-r12,sp,pc}

kernel_exit:
    @ r0: holds the task's stack pointer (passed as arg 0)
    @ Store kernel's registers onto kernel's stack.
    stmfd sp, {r4-r12,sp,lr}
    @ Restore the task's psr.
    @ r1: holds the task's PSR
    ldr r1, [r0, #-0x44]
    msr spsr, r1
    @ Load the task's pc into lr_svc.
    ldr lr, [r0, #-4]!
    @ Load the rest of the user registers.
    ldmea r0, {r0-r14}^
    @ Return from the exception (copies SPSR to CPSR).
    movs pc, lr

setup_swi:
    ldr r0, =vic8
    ldr r0, [r0]
    ldr r1, =0x08
    str r0, [r1]

    ldr r0, =kernel_entry
    ldr r1, =0x28
    str r0, [r1]
    mov pc, lr

vic8:
    ldr pc, [pc, #0x20]


@ TODO: Should these be implemented? Without them, they generate linker errors.
.global __aeabi_unwind_cpp_pr0
__aeabi_unwind_cpp_pr0:
.global __aeabi_unwind_cpp_pr1
__aeabi_unwind_cpp_pr1:
.global _ZN4core9panicking18panic_bounds_check17h9c1e1a81d6437172E
_ZN4core9panicking18panic_bounds_check17h9c1e1a81d6437172E:
.global _ZN4core9panicking5panic17h4f3d44b7d1aa30cbE
_ZN4core9panicking5panic17h4f3d44b7d1aa30cbE:
