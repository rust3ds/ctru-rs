use fmt;
use io::{self, Write};

// NOTE: We're just gonna use the spin mutex until we figure out how to properly
// implement mutexes with ctrulib functions
use spin::Mutex;
use libctru::libc;

pub static STDOUT: Mutex<StdoutRaw> = Mutex::new(StdoutRaw(()));

pub struct StdoutRaw(());

impl Write for StdoutRaw {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            // devkitPro's version of write(2) fails if zero bytes are written,
            // so let's just exit if the buffer size is zero
            if buf.is_empty() {
                return Ok(buf.len())
            }
            libc::write(libc::STDOUT_FILENO, buf.as_ptr() as *const _, buf.len());
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    STDOUT.lock().write_fmt(args).unwrap();
}
