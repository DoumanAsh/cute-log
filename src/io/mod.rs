#[cfg(feature="timestamp")]
fn get_date() -> impl ::std::fmt::Display {
    chrono::offset::Local::now().format("%F %H:%M:%S%.3f %z")
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
