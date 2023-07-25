// main.rs
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}

use lazy_static::lazy_static;
use bootloader_api::{config::Mapping, info::{MemoryRegionKind, MemoryRegion}};
use writer::FrameBufferWriter;
use spin::Mutex;
// use crate::writer::print;
use x86_64::instructions::hlt;

extern crate alloc;

use good_memory_allocator::SpinLockedAllocator;


#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

// static mut BOOT_INFO: Option<&'static mut bootloader_api::BootInfo> = None;

//Use the entry_point macro to register the entry point function: bootloader_api::entry_point!(kernel_main)

//optionally pass a custom config



pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();

    config.mappings.physical_memory = Some(Mapping::Dynamic);

    config.kernel_stack_size = 100 * 1024; // 100 KiB

    config
};

use core::{fmt::{Write, Arguments}};
// use std::println;
mod writer;
mod macros;
mod interrupts;



bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

lazy_static! {
    pub static ref FRAME_BUFFER: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);
}

#[no_mangle]
fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    //boot_info.framebuffer is our target for display

    
//    x86_64::instructions::interrupts::enable();
    
    // let frame_buffer_writer = unsafe { FRAMEBUFFER_WRITER.as_mut().unwrap().as_mut() };
    
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    *FRAME_BUFFER.lock() = Some(FrameBufferWriter::new(buffer, frame_buffer_info, 250, 100));
    // let mut data = FRAME_BUFFER.lock();
    
    // let frame_buffer_info =  boot_info.framebuffer.as_mut().unwrap().info();
    // let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    // let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    // write!(*data, "Testing testing {} and {}", 1, 4.0/2.0).unwrap();
    // // Move the write position horizontally by 10 pixels
    // frame_buffer_writer.move_horizontal(100);

    // // Move the write position vertically by 20 pixels
    // frame_buffer_writer.move_vertical(200);
    //let memory_regions_count = boot_info.memory_regions.iter().size_hint();
    //println!("{}", memory_regions_count.0);

    //Let's get the usable memory
    let last_memory_region = boot_info.memory_regions.last().unwrap();
    //println!("{:X}", last_memory_region.end);

    //get the first bootload memory
    let mut boot_loader_memory_region = MemoryRegion::empty();

    for memory_region in boot_info.memory_regions.iter() {
        match memory_region.kind {
            MemoryRegionKind::Bootloader => {
                boot_loader_memory_region = *memory_region;
                break;
            }
            _ => continue,
        }
    }

    let physical_memory_offset = boot_info.physical_memory_offset.into_option().unwrap();
    //let heap_start = 0x3E000 + physical_memory_offset;
    //let heap_size = 0x7FC2000;
    let heap_start = boot_loader_memory_region.end + 0x1 + physical_memory_offset;
    let heap_size = last_memory_region.end - (boot_loader_memory_region.end + 0x1);

    //println!("{:X} {:X}", heap_start as usize, heap_size as usize);

    unsafe {
        ALLOCATOR.init(heap_start as usize, heap_size as usize);
    }

    use alloc::boxed::Box;

    let x = Box::new(33);

    // writeln!(frame_buffer_writer, "Value in X is {}", x).unwrap();

    //Let's examine our memory
    //Go through memory regions passed and add usable ones to our global allocator
    /*let mut counter = 0 as u8;
    for memory_region in boot_info.memory_regions.iter() {
        counter += 1;
        frame_buffer_writer
            .write_fmt(format_args!("{}. ", counter)) //All other formatting macros (format!, write, println!, etc) are proxied through this one. format_args!, unlike its derived macros, avoids heap allocations.
            .unwrap();
        //print!("{}. ", counter);
        frame_buffer_writer
            .write_fmt(format_args!("{:X} ", memory_region.start)) //All other formatting macros (format!, write, println!, etc) are proxied through this one. format_args!, unlike its derived macros, avoids heap allocations.
            .unwrap();
        //print!("{:X}. ", memory_region.start);
        frame_buffer_writer
            .write_fmt(format_args!("{:X}, ", memory_region.end))
            .unwrap();
        //print!("{:X}. ", memory_region.end);
        frame_buffer_writer
            .write_fmt(format_args!(
                "size = {:X}, ",
                memory_region.end - memory_region.start
            ))
            .unwrap();
        //print!("size = {:X}, ", memory_region.end - memory_region.start);
        match memory_region.kind {
            MemoryRegionKind::Usable => write!(frame_buffer_writer, "Usable;  ").unwrap(),
            MemoryRegionKind::Bootloader => write!(frame_buffer_writer, "Bootload;").unwrap(),
            MemoryRegionKind::UnknownUefi(_) => {
                write!(frame_buffer_writer, "UnknownUefi;").unwrap();
            }
            MemoryRegionKind::UnknownBios(_) => {
                write!(frame_buffer_writer, "UnknownBios;").unwrap();
            }
            _ => write!(frame_buffer_writer, "Unknown;").unwrap(),
        }
    }*/

    println!("Welcome to YOMA OS");
    //println!("{:X} {:X} {:?}", boot_loader_memory_region.start, boot_loader_memory_region.end, boot_loader_memory_region.kind);
    println!("Hello, {}", "World!");
    // Write some more text at the updated position
    // write!(*data, "Additional text").unwrap();
    interrupts::init(); 
    println!("I could do much better than this");
    println!("I am this person");
    print!("\nValue could be 34");
    loop {
        hlt(); //stop x86_64 from being unnecessarily busy while looping
        
    }
}

// lazy_static! {
//     static ref FRAME_BUFFER_WRITER: FrameBufferWriter = {
//         // Access boot_info from the global static variable
//         let boot_info = unsafe { BOOT_INFO.expect("boot_info not initialized") };

//         // Obtain the necessary information from boot_info
//         let framebuffer = boot_info.framebuffer.as_mut().unwrap();
//         let frame_buffer_info = framebuffer.info();
//         let buffer = framebuffer.buffer_mut();

//         // Initialize and return the FrameBufferWriter instance
//         FrameBufferWriter::new(buffer, frame_buffer_info)
//     };
// }

// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => {
//         {
//             // use core::fmt::Write;
//             // use crate::writer::FRAME_BUFFER_WRITER;

//             let frame_buffer_writer = &*FRAME_BUFFER_WRITER;
//             write!(frame_buffer_writer, $($arg)*).unwrap();
//         }
//     };
// }

