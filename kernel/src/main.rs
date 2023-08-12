#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// static HELLO: &[u8] = b"Hello World!";

fn kernel_entry(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_entry);
