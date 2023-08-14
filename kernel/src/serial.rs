use x86::io::{inb, outb};

pub struct SerialPort {
    port: u16,
}

static init_complete_message: &[u8] = b"Serial port initialized!\n";
const SERIAL_PORT_RATE: u8 = (115200u32 / 9600) as u8;

impl SerialPort {
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

            if inb(port + 5) != 0xAE {
                // check if the serial port is faulty
                return None;
            }

            outb(port + 4, 0x0F);
        }

        let sp = SerialPort { port };
        sp.write_bytes(init_complete_message);

        Some(sp)
    }

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
