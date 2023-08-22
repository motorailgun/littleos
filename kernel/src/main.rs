#![no_std]
#![no_main]

#![feature(alloc_error_handler)]

mod serial;
mod allocator;

extern crate alloc;

use core::panic::PanicInfo;
use alloc::{alloc::Layout, vec::Vec};
use allocator::DumbAllocator;

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

#[global_allocator]
static mut DUMBALLOCATOR: DumbAllocator = unsafe {DumbAllocator::new_uninit()};

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
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

    let physical_memory_offset = match boot_info.physical_memory_offset.as_ref() {
        Some(offset) => *offset,
        None => 0,
    };

    println!("physical memory offset address: {:#x}", physical_memory_offset);

    let largest_region = boot_info.memory_regions.iter().filter(|region| {
            match region.kind {
                bootloader_api::info::MemoryRegionKind::Usable => true,
                _ => false,
            }
        }).reduce(|res, par| {
            let res_size = res.end - res.start;
            let par_size = par.end - par.start;

            if res_size > par_size { res } else { par }
        }).unwrap_or_else(|| {
            println!("no usable memory region found!");
            panic!();
        });

    unsafe {
        DUMBALLOCATOR.init_instance(
            (largest_region.start + physical_memory_offset) as usize, 
            (largest_region.end + physical_memory_offset) as usize)
    };


    let mut x = Vec::<usize>::new();
    x.push(1);
    x.push(2);

    println!("{:?}", x);

    loop {}
}

bootloader_api::entry_point!(kernel_entry, config = &BOOTLOADER_CONFIG);
