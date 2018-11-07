Byteunit
============

Byteunit is rust utilities to parse byte string into bytes count and format bytes count back into string.

## Installation

Put `byteunit` to your dependencies in `Cargo.toml` file:

```
[dependencies]
byteunit = "0.1.1"
```

## Usage Examples

```rust
extern crate byteunit;

use byteunit;

fn main() {
    let input = "1.23 MB";

    // Parse string into bytes
    let bytes: u64 = byteunit::parse(input).unwrap();
    assert_eq!(bytes, 1_230_000);

    // Format bytes into string
    let bytes_str = byteunit::format(bytes);
    assert_eq!(&bytes_str, input);

    // Parse to specific unit
    let kb: f64 = byteunit::parse_to(input, byteunit::Unit::KB).unwrap();
    assert_eq!(kb, 1_230 as f64);

    // Format to specific unit
    let kb_str = byteunit::format_to(bytes, byteunit::Unit::KB);
    assert_eq!(&kb_str, "1230 KB");
}
```