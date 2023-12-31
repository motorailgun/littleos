use core::fmt;
use spin::Mutex;
use core::cell::OnceCell;
use x86::io::{inb, outb};

#[derive(Copy, Clone)]
pub struct SerialPort {
    port: u16,
}

static INIT_COMPLETE_MESSAGE: &[u8] = b"Serial port initialized!\n";
const SERIAL_PORT_RATE: u8 = (115200u32 / 9600) as u8;

// serial port setup, basically based on OSDev wiki:
// lookup: https://wiki.osdev.org/Serial_Ports
pub fn new(port: u16) -> Option<SerialPort> {
    unsafe {
        outb(port + 1, 0x00); // Disable all interrupts
        outb(port + 3, 0x80); // Enable DLAB (set baud rate divisor)
        outb(port + 0, SERIAL_PORT_RATE); // Set divisor to 12 (lo byte) 115200 baud
        outb(port + 1, 0x00); //                   (hi byte)
        outb(port + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        outb(port + 4, 0x0B); // IRQs enabled, RTS/DSR set
        outb(port + 4, 0x1E); // Set in loopback mode, test the serial chip
        outb(port + 0, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        if inb(port + 0) != 0xAE {
            // check if the serial port is faulty
            return None;
        }

        // not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled
        outb(port + 4, 0x0F);
    }

    let sp = SerialPort { port };
    sp.write_bytes(INIT_COMPLETE_MESSAGE);

    Some(sp)
}

impl SerialPort {
    fn is_transmit_empty(&self) -> bool {
        (unsafe { inb(self.port + 5) & 0x20 } > 0)
    }

    fn write_byte(&self, char: u8) {
        while !self.is_transmit_empty() {}

        unsafe { outb(self.port, char) };
    }

    pub fn write_bytes<'a, T: IntoIterator<Item = &'a u8>>(&self, data: T) {
        for char in data.into_iter() {
            self.write_byte(*char)
        }
    }
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_bytes(s.as_bytes());

        Ok(())
    }
}


pub static WRITER: Mutex<OnceCell<SerialPort>> = Mutex::new(OnceCell::new());

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().get_mut().unwrap().write_fmt(args).unwrap();
}
