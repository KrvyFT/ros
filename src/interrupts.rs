use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handle);
        idt.double_fault.set_handler_fn(double_fault_handle);
        idt
    };
}

use crate::println;
pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handle(stack_fream: InterruptStackFrame) {
    println!("EXECPTION: BBREAKPOINT\n{:#?}", stack_fream);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

extern "x86-interrupt" fn double_fault_handle(
    stack_fream: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DPUBLE FAULT\n{:#?}", stack_fream);
}
