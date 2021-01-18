use core::{mem, ptr, cmp};
use core::fmt::{self, Write};

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(i32)]
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
//Re-check NDK sources, I think internally kernel limits to 4076, but
//it includes some overhead of logcat machinery, hence 4000
//Don't remember details
const BUFFER_CAPACITY: usize = 4000;
const DEFAULT_TAG: &[u8; 5] = b"Rust\0";

#[link(name = "log")]
extern "C" {
    pub fn __android_log_write(prio: i32, tag: *const i8, text: *const i8) -> i32;
}

pub struct Writer {
    //Null character is not within limit
    tag: mem::MaybeUninit<[u8; TAG_MAX_LEN + 1]>,
    prio: LogPriority,
    //Null character is not within limit
    buffer: mem::MaybeUninit<[u8; BUFFER_CAPACITY + 1]>,
    len: usize,
}

impl Writer {
    fn new(tag: mem::MaybeUninit<[u8; TAG_MAX_LEN + 1]>, prio: LogPriority) -> Self {
        Self {
            tag,
            prio,
            buffer: mem::MaybeUninit::uninit(),
            len: 0,
        }
    }

    #[inline(always)]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr() as _
    }

    #[inline(always)]
    fn last_flush(&mut self) {
        if self.len > 0 {
            self.flush();
        }
    }

    fn flush(&mut self) {
        unsafe {
            (self.buffer.as_mut_ptr() as *mut u8).add(self.len).write(0);
            __android_log_write(self.prio as _, self.tag.as_ptr() as _, self.buffer.as_ptr() as *const _);
        }
        self.len = 0;
    }

    #[inline]
    fn copy_text<'a>(&mut self, text: &'a str) -> &'a str {
        let write_len = cmp::min(BUFFER_CAPACITY.saturating_sub(self.len), text.len());
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), self.as_mut_ptr().add(self.len), write_len);
        }
        self.len += write_len;
        &text[write_len..]
    }

    fn write_text(&mut self, mut text: &str) {
        //At this point text.len() <= BUFFER_CAPACITY
        loop {
            text = self.copy_text(text);

            if text.len() == 0 {
                break;
            } else {
                self.flush();
            }
        }
    }
}

impl fmt::Write for Writer {
    #[inline]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.write_text(text);

        Ok(())
    }
}

impl Drop for Writer {
    #[inline]
    fn drop(&mut self) {
        self.last_flush();
    }
}

impl crate::Logger {
    #[inline]
    ///Logger printer.
    pub(crate) fn print(record: &log::Record) {
        let prio = match record.level() {
            log::Level::Warn => LogPriority::WARN,
            log::Level::Info => LogPriority::INFO,
            log::Level::Debug => LogPriority::DEBUG,
            log::Level::Error => LogPriority::ERROR,
            log::Level::Trace => LogPriority::VERBOSE,
        };

        let mut tag: mem::MaybeUninit<[u8; TAG_MAX_LEN+1]> = mem::MaybeUninit::uninit();
        match record.module_path() {
            Some(module) => match module.len() > TAG_MAX_LEN {
                true => unsafe {
                    ptr::copy_nonoverlapping(module.as_ptr(), tag.as_mut_ptr() as *mut u8, TAG_MAX_LEN);
                    (tag.as_mut_ptr() as *mut u8).add(TAG_MAX_LEN).write(0);
                },
                false => unsafe {
                    ptr::copy_nonoverlapping(module.as_ptr(), tag.as_mut_ptr() as *mut u8, module.len());
                    (tag.as_mut_ptr() as *mut u8).add(module.len()).write(0);
                }
            },
            None => unsafe {
                ptr::copy_nonoverlapping(DEFAULT_TAG.as_ptr(), tag.as_mut_ptr() as *mut u8, DEFAULT_TAG.len());
            },
        }

        let mut writer = Writer::new(tag, prio);

        let _ = write!(writer, "{{{}:{}}} - {}", record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0), record.args());
    }
}
