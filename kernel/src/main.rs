#![no_std]
#![no_main]

mod serial;
use core::panic::PanicInfo;
const SERIAL_PORT_ADDRESS: u16 = 0x3f8;

const BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();

    // set memory offset as 16TiB
    config.mappings.physical_memory = Some(bootloader_api::config::Mapping::FixedAddress(0x10 << 40));
    config
};

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn kernel_entry(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    serial::WRITER.lock().get_or_init(|| serial::new(SERIAL_PORT_ADDRESS).unwrap());
    println!("\n\n\tHello World!\n\n");


    println!("-- Memory regions info ---");
    boot_info.memory_regions.iter().for_each(|item| {
        let (start, end) = (item.start, item.end);
        let kind = item.kind;

        println!("Kind: {:?}, start = {:#x}, end = {:#x}", kind, start, end);
    });

    loop {}
}

bootloader_api::entry_point!(kernel_entry, config = &BOOTLOADER_CONFIG);
