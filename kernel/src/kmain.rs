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
        spin_sleep_ms(100);
        gpio16.clear();
        spin_sleep_ms(100);
    }
}

pub fn blink(times: u8, interval: u64) {
    use pi::timer::spin_sleep_ms;

    let mut gpio16 = pi::gpio::Gpio::new(16).into_output();
    for _ in 0..times {
       gpio16.set();
       spin_sleep_ms(interval);
       gpio16.clear();
       spin_sleep_ms(interval);
    }
}

pub fn echo() {
    use pi::timer::spin_sleep_ms;
    use pi::uart::MiniUart;

    let mut uart = MiniUart::new();

    loop {
        let byte = uart.read_byte();
        uart.write_byte(byte);
        blink(1, 10);
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    blink(3, 100);


    echo();
    // FIXME: Start the shell.
}
