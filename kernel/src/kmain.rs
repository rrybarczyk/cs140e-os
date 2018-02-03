#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]

extern crate pi;
extern crate stack_vec;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

pub fn blinky() {
    use pi::timer::spin_sleep_ms;

    let mut gpio16 = pi::gpio::Gpio::new(16).into_output();
    loop {
        gpio16.set();
        spin_sleep_ms(1000);
        gpio16.clear();
        spin_sleep_ms(1000);
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    loop {
        write_byte(read_byte())
        write_str("<-")
    }
    blinky();
    // FIXME: Start the shell.
}
