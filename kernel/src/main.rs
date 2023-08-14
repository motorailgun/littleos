#![no_std]
#![no_main]

mod serial;

use core::panic::PanicInfo;
use serial::SerialPort;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"\n\n\tHello World!\n\n";
const SERIAL_PORT_ADDRESS: u16 = 0x3f8;

fn kernel_entry(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let serial_port = SerialPort::new(SERIAL_PORT_ADDRESS).unwrap();
    serial_port.write_bytes(HELLO);

    loop {}
}

bootloader_api::entry_point!(kernel_entry);
