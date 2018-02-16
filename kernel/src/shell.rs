use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }

    pub fn exec(&self) {
        match self.path() {
            "echo" => {
                let len = self.args.len();
                for i in 1..(len - 1) {
                    kprint!("{} ", self.args.as_slice()[i]);
                }
                kprint!("{}", self.args.as_slice()[len - 1]);
            }
            cmd => {
                kprint!("unknown command: {}", cmd)
            }
        
        }
        kprintln!("");
    }
}

const BS: u8 = 0x08;
const BEL: u8 = 0x07;
const LF: u8 = 0x0A;
const CR: u8 = 0x0D;
const DEL: u8 = 0x7F;
fn read_line(buf: &mut [u8]) -> &str {
    use std::str;
    let mut read = 0;

    loop {
        let b = CONSOLE.lock().read_byte();
        match b {
            BS | DEL if read > 0 => {
                kprint!("{}", BS as char);
                kprint!(" ");
                kprint!("{}", BS as char);
                read -= 1;
            }
            LF | CR => {
                kprintln!("");
                break;
            }
            _ if read == buf.len() => {
                kprint!("{}", BEL as char);
            }
            byte @ b' ' ... b'~' => {
                kprint!("{}", byte as char);
                buf[read] = byte;
                read += 1;
            }
            _ => {
                kprint!("{}", BEL as char);
            }
        }
    }
    str::from_utf8(&buf[..read]).unwrap()
}

const MAXBUF: usize = 512;
const MAXARGS: usize = 64;
/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    loop {
        kprint!("{}", prefix);
        match Command::parse(read_line(&mut [0u8; MAXBUF]),
                             &mut [""; MAXARGS]) {
            Ok(cmd) => cmd.exec(),
            Err(Error::TooManyArgs) => kprintln!("error: too many arguments"),
            Err(Error::Empty) => { }
        }
    }
}
