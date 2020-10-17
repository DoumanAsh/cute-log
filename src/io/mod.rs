#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_os = "android")]
mod android;

#[cfg(not(any(target_os = "android", target_arch = "wasm32", target_os = "unknown")))]
mod regular;

#[cfg(all(not(target_arch = "wasm32"), target_os = "unknown"))]
mod unknown;
