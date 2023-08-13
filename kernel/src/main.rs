#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// static HELLO: &[u8] = b"\n\nHello World!\n\n";

fn output_serial<T>(port: u16, data: T) -> ()
where T: IntoIterator<Item = &'static u8> {
    for char in data.into_iter() {
        unsafe { x86::io::outb(port, *char) }
    }
}

// initializes serial port
fn setup_serial() -> () {
    let port = 0x3f8;
    let init_complete_message = b"Serial port initialized!\n";

    unsafe {
        // from xv6-public
        // uart.c line 20, uartinit()
        use x86::io::{outb, inb};

        outb(port + 2, 0);
        outb(port + 3, 0x80);
        outb(port + 0, (115200/9600) as u8);
        outb(port + 1, 0);
        outb(port + 3, 0x03);
        outb(port + 4, 0);
        // outb(port + 1, 1);

        if inb(port + 5) == 0xFF { return }

        output_serial(port, init_complete_message)
    }
}

fn kernel_entry(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    setup_serial();    

    loop {}
}

bootloader_api::entry_point!(kernel_entry);
