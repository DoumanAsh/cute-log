use std::os::raw::{c_int, c_char};
use core::{ptr, mem, cmp};
use core::fmt::{self, Write};

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(isize)]
enum LogPriority {
    UNKNOWN = 0,
    DEFAULT = 1,
    VERBOSE = 2,
    DEBUG = 3,
    INFO = 4,
    WARN = 5,
    ERROR = 6,
    FATAL = 7,
    SILENT = 8,
}

const TAG_MAX_LEN: usize = 23;
const MSG_MAX_LEN: usize = 4000;

#[link(name = "log")]
extern "C" {
    pub fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

pub struct Writer {
    buffer: [u8; MSG_MAX_LEN + 1],
    len: usize,
}

impl Writer {
    fn new() -> Self {
        Self {
            buffer: unsafe { mem::MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    pub fn write(&mut self, prio: c_int, tag: *const c_char) {
        unsafe {
            *(self.buffer.as_mut_ptr().add(self.len)) = 0;
            __android_log_write(prio, tag, self.buffer.as_ptr() as *const c_char);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let text_len = text.len();
        let remaining_len = MSG_MAX_LEN - self.len;

        if remaining_len > 0 {
            let write_len = cmp::min(text_len, remaining_len);
            unsafe {
                ptr::copy_nonoverlapping(text.as_ptr(), self.buffer.as_mut_ptr().add(self.len), write_len);
            }
            self.len += write_len;
        }

        Ok(())
    }
}

pub fn print(record: &log::Record) {
    let prio = match record.level() {
        log::Level::Warn => LogPriority::WARN,
        log::Level::Info => LogPriority::INFO,
        log::Level::Debug => LogPriority::DEBUG,
        log::Level::Error => LogPriority::ERROR,
        log::Level::Trace => LogPriority::VERBOSE,
    };

    //Null character is not within limit
    let mut tag: [u8; TAG_MAX_LEN + 1] = unsafe { mem::MaybeUninit::uninit().assume_init() };
    match record.module_path() {
        Some(module) => match module.len() > TAG_MAX_LEN {
            true => unsafe {
                ptr::copy_nonoverlapping(module.as_ptr(), tag.as_mut_ptr(), TAG_MAX_LEN);
                tag.as_mut_ptr().add(TAG_MAX_LEN).write(0);
            },
            false => unsafe {
                ptr::copy_nonoverlapping(module.as_ptr(), tag.as_mut_ptr(), module.len());
                tag.as_mut_ptr().add(module.len()).write(0);
            }
        },
        None => unsafe {
            const TAG: &[u8; 5] = b"Rust\0";
            ptr::copy_nonoverlapping(TAG.as_ptr(), tag.as_mut_ptr(), TAG.len());
        },
    }

    let mut writer = Writer::new();

    #[cfg(feature="timestamp")]
    let _ = write!(writer, "{:<5} [{}] - {} at {}:{}", record.level(), super::get_date(), record.args(), record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0));

    #[cfg(not(feature="timestamp"))]
    let _ = write!(writer, "{:<5} - {} at {}:{}", record.level(), record.args(), record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0));

    writer.write(prio as c_int, tag.as_ptr() as *const c_char);
}
