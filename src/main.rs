#![no_std]
#![no_main]

use kls_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kls_os::init();

    for i in 1..=30 {
        println!("klsOS - {}", i);
    }

    loop {}
}