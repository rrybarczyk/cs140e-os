#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(exclusive_range_pattern)]
#![feature(alloc, allocator_api, global_allocator)]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;
extern crate pi;
extern crate stack_vec;

pub mod allocator;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

#[cfg(not(test))]
use allocator::Allocator;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub fn blink(repeat: u8, interval: u64) {
    use pi::timer::spin_sleep_ms;

    let mut gpio16 = pi::gpio::Gpio::new(16).into_output();
    for _ in 0..repeat {
       gpio16.set();
       spin_sleep_ms(interval);
       gpio16.clear();
       spin_sleep_ms(interval);
    }
}

pub fn echo() {
    loop {
        let byte = console::CONSOLE.lock().read_byte();
        console::CONSOLE.lock().write_byte(byte);
        blink(1, 10);
    }
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn kmain() {
//    ALLOCATOR.initialize();
    console::kprintln!("PRAISE THE SUN!\nMAY THE FLAME GUIDE THEE\n");
    blink(3, 100);
    shell::shell("> ");
}
