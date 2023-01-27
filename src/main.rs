#![no_std]  // do not link the rust stdlib
#![no_main] // disable rust entry points

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] // disable name mangling of this function
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function
    // named '_start' by default.
    println!("Hello World{}", "!");

    loop {}
}

// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
