#![no_std]
#![feature(abi_x86_interrupt)]

pub mod panic;
pub mod interrupts;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_idt();
}