#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// rename the functions created by test_runner, which is usually 
// `main`. We can't use `main` because we specified `no_main`
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    // exit qemu
    exit_qemu(QemuExitCode::Success);
}

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[no_mangle]
pub extern  "C" fn _start() -> ! {
    for i in 0..100  {
        println!("Hello world using macros. Counter: {}", i);
        
    };

    #[cfg(test)]
    test_main();

    loop{}
}


// tests
#[test_case]
fn trivial_assertion() {
    print!("Trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

// This is to make sure that qemu exits once the tests finish
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

