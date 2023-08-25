extern crate alloc;
use alloc::vec::Vec;

pub unsafe fn pci_config_read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let l_bus = (bus as u32) << 16;
    let l_slot = (slot as u32) << 11;
    let l_func = (func as u32) << 8;
    let masked_offset = (offset & 0xfc) as u32;

    let address = l_bus | l_slot | l_func | masked_offset | 0x80000000;

    x86::io::outl(0xcf8, address);
    (0xffff & (x86::io::inl(0xcfc) >> ((offset & 2) * 8))) as u16
}

pub unsafe fn pcie_list_device() -> Vec<(u16, u16)> {
    let mut devices: Vec<(u16, u16)> = Vec::new();

    for bus in (0..).into_iter() {
        let mut flag = false;
        for slot in (0..).into_iter() {
            let vendor = pci_config_read_word(bus, slot, 0, 0);
            let device = pci_config_read_word(bus, slot, 0, 2);

            if vendor == 0xffff {flag = true; break;}

            devices.push((vendor, device));
        }

        if flag {break;}
    }

    devices
}
