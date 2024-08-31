#![no_std]
#![no_main]

mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer: *mut u8 = 0xb8000 as *mut u8;

    for (index, &byte) in b"klsOS".into_iter().enumerate() {
        unsafe {
            *vga_buffer.offset(index as isize * 2) = byte;
            *vga_buffer.offset(index as isize * 2 + 1) = 0x2;
        }
    }

    loop {}
}