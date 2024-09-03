use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Panic: {}", _info);
    
    loop {}
}