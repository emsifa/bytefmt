bytefmt
============

[![bytefmt on travis](https://travis-ci.org/emsifa/bytefmt.svg?branch=master)](https://travis-ci.org/emsifa/bytefmt)
[![bytefmt on crates.io](https://img.shields.io/crates/v/bytefmt.svg)](https://crates.io/crates/bytefmt)
[![bytefmt on docs.rs](https://docs.rs/bytefmt/badge.svg)](https://docs.rs/bytefmt)

Bytefmt is Rust utility to parse byte string into bytes count and vice versa.

## Installation

Put `bytefmt` to your dependencies in `Cargo.toml` file:

```
[dependencies]
bytefmt = "0.1.7"
```

## Usage Examples

```rust
extern crate bytefmt;

fn main() {
    let input = "1.23 MB";

    // Parse string into bytes
    let bytes: u64 = bytefmt::parse(input).unwrap();
    assert_eq!(bytes, 1_230_000);

    // Format bytes into string
    let bytes_str = bytefmt::format(bytes);
    assert_eq!(&bytes_str, input);

    // Parse to specific unit
    let kb: f64 = bytefmt::parse_to(input, bytefmt::Unit::KB).unwrap();
    assert_eq!(kb, 1_230 as f64);

    // Format to specific unit
    let kb_str = bytefmt::format_to(bytes, bytefmt::Unit::KB);
    assert_eq!(&kb_str, "1230 KB");
}
```