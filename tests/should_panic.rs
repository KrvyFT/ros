#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use ros::{exit_qemu, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("test did not panic");
        exit_qemu(ros::QemuExitCode::Failed);
    }
    exit_qemu(ros::QemuExitCode::Success);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[OK]");
    exit_qemu(ros::QemuExitCode::Success);
    loop {}
}

#[test_case]
fn should_fail() {
    serial_print!("should_fail... ");
    assert_eq!(1, 0);
}
