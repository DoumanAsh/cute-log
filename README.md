# cute-log

[![Build Status](https://travis-ci.org/DoumanAsh/cute-log.svg?branch=master)](https://travis-ci.org/DoumanAsh/cute-log)
[![Crates.io](https://img.shields.io/crates/v/cute-log.svg)](https://crates.io/crates/cute-log)
[![Documentation](https://docs.rs/cute-log/badge.svg)](https://docs.rs/crate/cute-log/)

Simple and cute log

## Usage

```rust
#[macro_use]
extern crate log;
extern crate cute_log;

fn main() {
    cute_log::init();
    info!("it works!");
}
```

## Available features

- `timestamp` - Enables timestamp in logs. Enabled by default.
- `color` - Enables coloring of log level. Enabled by default, but not for `wasm32` target

## Log level control

The logger is made without any builtin filters.

You can either control logs through compile time features of `log` crate.
Or use `set_max_level`.
