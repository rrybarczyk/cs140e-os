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
#![feature(pointer_methods)]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;
extern crate pi;
extern crate stack_vec;
extern crate fat32;

pub mod allocator;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;
pub mod fs;

#[cfg(not(test))]
use allocator::Allocator;
use console::{kprintln};
use fs::FileSystem;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

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

pub fn panic_test(n : usize) {
    match n {
        0 => {
            let test : Option<u64> = None; 
            test.unwrap();
        }
        1 => unreachable!(),
        2 => panic!(),
        _ => {}
    }
}

pub fn print_atag() {
    for atag in pi::atags::Atags::get() {
        kprintln!("{:#?}", atag);
    }
}

pub fn allocator_test() {
    kprintln!("{:?}", ALLOCATOR);
    {
        let mut v = vec![];
        for i in 0..1000 {
            v.push(i);
        }
        kprintln!("{:?}", ALLOCATOR);
    }
    kprintln!("{:?}", ALLOCATOR);

}

pub fn list_root() {
    use fat32::traits::{FileSystem,Dir,Entry};

    let root = FILE_SYSTEM.open_dir("/").unwrap();
    for e in root.entries().unwrap() {
        kprintln!("{}", e.name());
    }
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn kmain() {
    kprintln!("PRAISE THE SUN!");

    ALLOCATOR.initialize();
    FILE_SYSTEM.initialize();

//    let hello = String::from("MAY THE FLAME GUIDE THEE!");
//    kprintln!("{}", hello);

//    kprintln!("{:?}", ALLOCATOR);

    shell::shell("> ");
}
