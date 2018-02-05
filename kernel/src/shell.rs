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
        self.args.as_slice()[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    use std::io::Read;
    unimplemented!();

//    loop {
//        kprint!("{}", prefix);
//        let mut buf = [0u8; 512];
//        let mut console = CONSOLE.lock();
//        match (*console).read(&mut buf) {
//            Ok(read) => {
//                let mut argstring = unsafe {String::from_raw_parts(&mut buf[0], read, buf.len())};
//                let mut vecBuf = [""; 512];
//                let cmd = Command::parse(&argstring.as_str(), &mut vecBuf);
//                kprintln!("");
//                continue;
//            }
//            _ => {kprintln!(""); continue}
//        }
//    }
}
