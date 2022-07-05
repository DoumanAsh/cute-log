# cute-log

[![Rust](https://github.com/DoumanAsh/cute-log/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/cute-log/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/cute-log.svg)](https://crates.io/crates/cute-log)
[![Documentation](https://docs.rs/cute-log/badge.svg)](https://docs.rs/crate/cute-log/)

Simple and cute log

## Usage

```rust
fn main() {
    const LOGGER: cute_log::Logger = cute_log::Logger::new();
    LOGGER.set_max_level(cute_log::log::LevelFilter::Info);
    let _ = LOGGER.set_logger();
    log::info!("it works!");
}
```

## Available features

- `timestamp` - Enables timestamp in logs. Enabled by default.
- `std` - Enables use of `std` feature to provide `RUST_LOG` handling.

## Log level control

The logger is made without any builtin filters.

You can either control logs through compile time features of `log` crate.
Or use `set_max_level`.

## Supported platforms

- Android - via NDK logging library, therefore it must be linked.
- Wasm - via web console API.
- Any other platform with `std` available.
