use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};
use std::path::PathBuf;
use super::FILE_SYSTEM;
use fat32::traits::{FileSystem, Dir, Entry};
use std::io;
use std::str;

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

    fn cat(&self, pwd: &mut PathBuf) -> Result<(), ()> {
        use std::io::{Read, Write};
        let dir_buf = PathBuf::from(self.args[1]);
//        let dir_buf = if self.args.len() < 1 {
//            pwd.clone()
//        } else {
//            PathBuf::from(args[0])
//        };

                kprintln!("here\n");
        let abs_path = {
            if dir_buf.is_absolute() {
                dir_buf 
            } else {
                let mut tmp_pwd = pwd.clone();
                tmp_pwd.push(dir_buf);
                tmp_pwd
            }
        };

                kprintln!("here\n");
        FILE_SYSTEM.open(abs_path)
            .and_then(|dir_entry| 
                      dir_entry.into_file().ok_or(io::Error::new(io::ErrorKind::Other, 
                                                                "Is a directory")))
            .and_then(|mut file| {
                kprintln!("here\n");
                let mut buf = Vec::new();
                match file.read_to_end(&mut buf) {
                    Ok(n) => {
                        let s = str::from_utf8(&buf).unwrap();
                        kprintln!("{}", s);
                    }
                    _ => {}
                }
            
                Ok(())
            
            });
        Ok(())
    }

    fn cd(&self, pwd: &mut PathBuf) -> Result<(), ()> {
        let dir_buf = PathBuf::from(if self.args.len() < 1 {
            "/" 
        } else {
            self.args[1]
        });
        
        let abs_path = {
            if dir_buf.is_absolute() {
                dir_buf 
            } else {
                let mut tmp_pwd = pwd.to_path_buf();
                tmp_pwd.push(dir_buf);
                tmp_pwd
            }
        };
        
        let dir_entry = FILE_SYSTEM.open(abs_path.clone());
        if dir_entry.is_err() {
            kprintln!("cd: no such file or directory: {}", self.args[1]);
            return Err(());
        }

        if dir_entry.unwrap().as_dir().is_some() {
            pwd.set_file_name(abs_path);
            return Ok(());
        } else {
            kprintln!("cd: not a directory: {}", self.args[1]);
            return Err(());
        }
    }

    fn ls(&self, pwd: &mut PathBuf) -> Result<(),()> {
        let mut args = &self.args[1..];
        let all = args.get(0).and_then(|&arg| {
            if arg == "-a" {
                args = &args[1..];
                Some(true)
            } else {
                None
            }
        }).unwrap_or(false);

        let dir_buf = if args.len() < 1 {
            pwd.clone()
        } else {
            PathBuf::from(args[0])
        };
        
        let abs_path = {
            if dir_buf.is_absolute() {
                dir_buf 
            } else {
                let mut tmp_pwd = pwd.clone();
                tmp_pwd.push(dir_buf);
                tmp_pwd
            }
        };
    
        FILE_SYSTEM.open(abs_path.clone())
            .and_then(|dir_entry| 
                      dir_entry.into_dir().ok_or(io::Error::new(io::ErrorKind::Other, 
                                                                "not dir")))
            .and_then(|dir| dir.entries())
            .and_then(|entries| {
                for e in entries {
                    kprintln!("{}", e.name());
                }
                Ok(())
            })
            .map_err(|e| {
                match e.kind() {
                    io::ErrorKind::Other => kprintln!("ls: not supported {}", args[0]),
                    io::ErrorKind::NotFound => kprintln!("ls: no such file or directory: {}", args[0]),
                    _ => {},
                }
                ()
            })
    }

    pub fn exec(&self, pwd: &mut PathBuf) {
        match self.path() {
            "echo" => {
                let len = self.args.len();
                for i in 1..(len - 1) {
                    kprint!("{} ", self.args.as_slice()[i]);
                }
                kprintln!("{}", self.args.as_slice()[len - 1]);
                Ok(())
            }
            "ls" => self.ls(pwd),
            "pwd" => { kprintln!("{}", pwd.to_str().unwrap()); Ok(()) }
            "cd" => self.cd(pwd),
            "cat" => self.cat(pwd),
            cmd => {
                kprintln!("unknown command: {}", cmd);
                Ok(())
            }
        
        };
//        kprintln!("");
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
    let mut pwd = PathBuf::from("/");

    loop {
        kprint!("{} {}", pwd.to_str().unwrap(), prefix);
        match Command::parse(read_line(&mut [0u8; MAXBUF]),
                             &mut [""; MAXARGS]) {
            Ok(cmd) => cmd.exec(&mut pwd),
            Err(Error::TooManyArgs) => kprintln!("error: too many arguments"),
            Err(Error::Empty) => { }
        }
    }
}
