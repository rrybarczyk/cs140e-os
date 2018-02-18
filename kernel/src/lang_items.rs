use console::{kprint, kprintln};
const OVERDONE_PIE : &str = 
"
             (
       (      )     )
         )   (    (
        (          `
    .-\"\"^\"\"\"^\"\"^\"\"\"^\"\"-.
  (//\\\\//\\\\//\\\\//\\\\//\\\\//)
   ~\\^^^^^^^^^^^^^^^^^^/~
     `================`

    The pi is overdone.

";

#[no_mangle]
#[cfg(not(test))]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: ::std::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
    kprint!("{}", OVERDONE_PIE);
    kprintln!("---------- PANIC ----------");
    kprintln!("FILE: {}", file);
    kprintln!("LINE: {}", line);
    kprintln!("COL: {}", col);
    kprintln!("");
    kprintln!("{:?}", fmt);

    loop { unsafe { asm!("wfe") } }
}

#[cfg(not(test))] #[lang = "eh_personality"] pub extern fn eh_personality() {}
