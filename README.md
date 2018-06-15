# cute-log

[![Build Status](https://travis-ci.org/DoumanAsh/cute-log.svg?branch=master)](https://travis-ci.org/DoumanAsh/cute-log)
[![Crates.io](https://img.shields.io/crates/v/cortex-m-log.svg)](https://crates.io/crates/cortex-m-log)
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

- `timestamp` - Enables timestamp in logs.
