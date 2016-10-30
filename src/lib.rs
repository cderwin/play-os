#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate multiboot2;
extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;

mod utils;

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: ::core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPanic in {} at line {}:\n    {}", file, line, fmt);

    loop{}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {loop{}}


#[no_mangle]
pub extern fn rust_main(multiboot_addr: usize) {
    // small stack: 64 bytes
    
    vga_buffer::clear_screen();
    println!("Hello, world!");

    // debug info
    let boot_info = unsafe { multiboot2::load(multiboot_addr) };
    let mem_map_tag = boot_info.memory_map_tag()
        .expect("No multiboot2 memory map tag");

    println!("Memory areas: ");
    for area in mem_map_tag.memory_areas() {
        println!("    start: 0x{:x}, len: 0x{:x}", area.base_addr, area.length);
    }

    /*
    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("No multiboot2 elf sections tag");

    println!("\n\nKernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                 section.addr, section.size, section.flags);
    }
    */
    // never return
    panic!("Kernel finished.");
    loop{}
}
