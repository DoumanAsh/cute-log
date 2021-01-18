use wasm_bindgen::prelude::wasm_bindgen;

use core::{mem, ptr, cmp};
use core::fmt::{self, Write};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
}

const BUFFER_CAPACITY: usize = 4096;

pub struct Console {
    fun: fn(&str),
    buffer: mem::MaybeUninit<[u8; BUFFER_CAPACITY]>,
    len: usize,
}

impl Console {
    fn new(fun: fn(&str)) -> Self {
        Self {
            fun,
            buffer: mem::MaybeUninit::uninit(),
            len: 0,
        }
    }

    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr() as _
    }

    #[inline(always)]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr() as _
    }

    #[inline(always)]
    fn as_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.as_ptr(), self.len)
        }
    }

    #[inline(always)]
    fn last_flush(&mut self) {
        if self.len > 0 {
            self.flush();
        }
    }

    fn flush(&mut self) {
        let text = unsafe {
            core::str::from_utf8_unchecked(self.as_slice())
        };
        (self.fun)(text);
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

impl fmt::Write for Console {
    #[inline]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.write_text(text);

        Ok(())
    }
}

impl Drop for Console {
    #[inline]
    fn drop(&mut self) {
        self.last_flush();
    }
}

impl crate::Logger {
    #[inline]
    ///Logger printer.
    pub(crate) fn print(record: &log::Record) {
        let mut console = match record.level() {
            log::Level::Trace => Console::new(debug),
            log::Level::Debug => Console::new(debug),
            log::Level::Info => Console::new(info),
            log::Level::Warn => Console::new(warn),
            log::Level::Error => Console::new(error),
        };

        let _ = write!(console, "{:<5} {{{}:{}}} - {}", record.level(), record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0), record.args());
    }
}
