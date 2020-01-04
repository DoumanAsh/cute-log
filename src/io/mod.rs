pub struct TimeDate(time::PrimitiveDateTime);

impl core::fmt::Display for TimeDate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}-{:02}-{:02} {:02}:{:02}:{:02}", self.0.year(), self.0.month(), self.0.day(), self.0.hour(), self.0.minute(), self.0.second())
    }
}

#[cfg(feature="timestamp")]
#[inline]
fn get_date() -> impl core::fmt::Display {
    TimeDate(time::PrimitiveDateTime::now())
}

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_os = "android")]
mod android;
#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
mod regular;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
#[cfg(target_os = "android")]
pub use android::*;
#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
pub use regular::*;
