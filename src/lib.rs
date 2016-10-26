#![feature(lang_items)]
#![no_std]

extern crate rlibc;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {
    unsafe {
        print_str(b"Panic! (to the disco)", 0x4f, 0);
    };

    loop{}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {loop{}}


#[no_mangle]
pub extern fn rust_main() {
    // small stack: 64 bytes
    
    unsafe {
        print_str(b"Hello, world!", 0x1f, 1988);
    };

    panic!();
    
    // never return
    loop{}
}

const VGA_BUFFER_PTR: u32 = 0xb8000;

unsafe fn print_str(string: &[u8], color_byte: u8, offset: u32) {
    let base_ptr = VGA_BUFFER_PTR + offset;

    for (i, chr) in string.into_iter().enumerate() {
        let ptr = (base_ptr + 2*i as u32) as *mut _;
        *ptr = [*chr, color_byte];
    }
}
