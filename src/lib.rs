//! # bytefmt
//!
//! Bytefmt is Rust utility to parse byte string into bytes count and vice versa.
//!
//! ## Examples
//!
//! ```
//! extern crate bytefmt;
//!
//! fn main() {
//!     let input = "1.23 MB";
//!
//!     // Parse string into bytes
//!     let bytes: u64 = bytefmt::parse(input).unwrap();
//!     assert_eq!(bytes, 1_230_000);
//!
//!     // Format bytes into string
//!     let bytes_str = bytefmt::format(bytes);
//!     assert_eq!(&bytes_str, input);
//!
//!     // Parse to specific unit
//!     let kb: f64 = bytefmt::parse_to(input, bytefmt::Unit::KB).unwrap();
//!     assert_eq!(kb, 1_230 as f64);
//!
//!     // Format to specific unit
//!     let kb_str = bytefmt::format_to(bytes, bytefmt::Unit::KB);
//!     assert_eq!(&kb_str, "1230 KB");
//! }
//! ```
////////////////////////////////////////////////////////////////////////////////
extern crate regex;

use regex::Regex;

pub const B: u64 = 1;
pub const KB: u64 = 1_000;
pub const MB: u64 = 1_000_000;
pub const GB: u64 = 1_000_000_000;
pub const TB: u64 = 1_000_000_000_000;
pub const PB: u64 = 1_000_000_000_000_000;

pub const KIB: u64 = 1_024;
pub const MIB: u64 = 1_048_576;
pub const GIB: u64 = 1_073_741_824;
pub const TIB: u64 = 1_099_511_627_776;
pub const PIB: u64 = 1_125_899_906_842_624;

#[derive(Debug,PartialEq)]
pub enum Unit {
    B,
    KB,
    MB,
    GB,
    TB,
    PB,
    KIB,
    MIB,
    GIB,
    TIB,
    PIB,
}

fn parse_size_unit<S: Into<String>>(s: S) -> Result<(f64, Unit), &'static str> {
    let str = s.into();
    let re = Regex::new(r"^(?i)(\d+(\.\d+)?) *((k|m|g|t|p|ki|mi|gi|ti|pi)?b)?$").unwrap();
    let captures = re.captures(&str);
    
    match captures {
        Some(res) => {
            let size = res[1].to_owned();
            let unit: String = match res.get(3) {
                Some(val) => val.as_str().to_owned().to_uppercase(),
                None => "B".to_owned(),
            };
            
            Ok((size.parse::<f64>().unwrap(), match &*unit {
                "B" => Unit::B,
                "KB" => Unit::KB,
                "MB" => Unit::MB,
                "GB" => Unit::GB,
                "TB" => Unit::TB,
                "PB" => Unit::PB,
                "KIB" => Unit::KIB,
                "MIB" => Unit::MIB,
                "GIB" => Unit::GIB,
                "TIB" => Unit::TIB,
                "PIB" => Unit::PIB,
                _ => Unit::B,
            }))
        }
        None => Err("Parse Error. Invalid byte format."),
    }
}

/// Parse given string to bytes count
///
/// # Examples  
///
/// ```
/// assert_eq!(bytefmt::parse("123").unwrap(), 123);
/// assert_eq!(bytefmt::parse("1.23 B").unwrap(), 1);
/// assert_eq!(bytefmt::parse("1.23 KB").unwrap(), 1_230);
/// assert_eq!(bytefmt::parse("1.23 MB").unwrap(), 1_230_000);
/// assert_eq!(bytefmt::parse("1.23 GB").unwrap(), 1_230_000_000);
/// assert_eq!(bytefmt::parse("1.23 TB").unwrap(), 1_230_000_000_000);
/// assert_eq!(bytefmt::parse("1.23 PB").unwrap(), 1_230_000_000_000_000);
/// assert_eq!(bytefmt::parse("1.23 KiB").unwrap(), 1_259);
/// assert_eq!(bytefmt::parse("1.23 MiB").unwrap(), 1_289_748);
/// assert_eq!(bytefmt::parse("1.23 GiB").unwrap(), 1_320_702_443);
/// assert_eq!(bytefmt::parse("1.23 TiB").unwrap(), 1_352_399_302_164);
/// assert_eq!(bytefmt::parse("1.23 PiB").unwrap(), 1_384_856_885_416_427);
/// ```
pub fn parse<S: Into<String>>(str: S) -> Result<u64, &'static str> {
    let parsed = parse_size_unit(str);

    match parsed {
        Ok(r) => {
            let value = r.0;
            let unit = r.1;
            
            let bytes = match unit {
                Unit::B => value * B as f64,
                Unit::KB => value * KB as f64,
                Unit::MB => value * MB as f64,
                Unit::GB => value * GB as f64,
                Unit::TB => value * TB as f64,
                Unit::PB => value * PB as f64,
                Unit::KIB => value * KIB as f64,
                Unit::MIB => value * MIB as f64,
                Unit::GIB => value * GIB as f64,
                Unit::TIB => value * TIB as f64,
                Unit::PIB => value * PIB as f64,
            };

            Ok(bytes as u64)
        },
        Err(msg) => Err(msg),
    }
}

/// Parse given string to specific byte unit
///
/// # Examples  
///
/// ```
/// let kb = bytefmt::parse_to("123B", bytefmt::Unit::KB).unwrap();
/// let mb = bytefmt::parse_to("123B", bytefmt::Unit::MB).unwrap();
/// 
/// assert_eq!(kb, 0.123);
/// assert_eq!(mb, 0.000123);
/// ```
pub fn parse_to<S: Into<String>>(str: S, result_unit: Unit) -> Result<f64, &'static str> {
    match parse(str) {
        Ok(bytes) => {
            let result = match result_unit {
                Unit::B => bytes as f64,
                Unit::KB => bytes as f64 / KB as f64,
                Unit::MB => bytes as f64 / MB as f64,
                Unit::GB => bytes as f64 / GB as f64,
                Unit::TB => bytes as f64 / TB as f64,
                Unit::PB => bytes as f64 / PB as f64,
                Unit::KIB => bytes as f64 / KIB as f64,
                Unit::MIB => bytes as f64 / MIB as f64,
                Unit::GIB => bytes as f64 / GIB as f64,
                Unit::TIB => bytes as f64 / TIB as f64,
                Unit::PIB => bytes as f64 / PIB as f64,
            };

            Ok(result)
        },
        Err(msg) => Err(msg),
    }
}

/// Format bytes to byte string
///
/// # Examples
///
/// ```
/// assert_eq!(bytefmt::format(123), "123 B");
/// assert_eq!(bytefmt::format(1_230), "1.23 KB");
/// assert_eq!(bytefmt::format(1_230_000), "1.23 MB");
/// assert_eq!(bytefmt::format(1_230_000_000), "1.23 GB");
/// assert_eq!(bytefmt::format(1_230_000_000_000), "1.23 TB");
/// assert_eq!(bytefmt::format(1_230_000_000_000_000), "1.23 PB");
/// ```
pub fn format(bytes: u64) -> String {
    if bytes < KB {
        return format_to(bytes, Unit::B);
    }

    if bytes < MB {
        return format_to(bytes, Unit::KB);
    }

    if bytes < GB {
        return format_to(bytes, Unit::MB);
    }

    if bytes < TB {
        return format_to(bytes, Unit::GB);
    }

    if bytes < PB {
        return format_to(bytes, Unit::TB);
    }

    format_to(bytes, Unit::PB)
}

/// Format bytes to specific unit byte string
///
/// # Examples
///
/// ```
/// assert_eq!(bytefmt::format_to(1245, bytefmt::Unit::KB), "1.25 KB");
/// assert_eq!(bytefmt::format_to(1275, bytefmt::Unit::KIB), "1.25 KiB");
/// assert_eq!(bytefmt::format_to(500, bytefmt::Unit::KB), "0.5 KB");
/// assert_eq!(bytefmt::format_to(512, bytefmt::Unit::KIB), "0.5 KiB");
/// ```
pub fn format_to(bytes: u64, unit: Unit) -> String {
    let result = match unit {
        Unit::B => bytes as f64,
        Unit::KB => bytes as f64 / KB as f64,
        Unit::MB => bytes as f64 / MB as f64,
        Unit::GB => bytes as f64 / GB as f64,
        Unit::TB => bytes as f64 / TB as f64,
        Unit::PB => bytes as f64 / PB as f64,
        Unit::KIB => bytes as f64 / KIB as f64,
        Unit::MIB => bytes as f64 / MIB as f64,
        Unit::GIB => bytes as f64 / GIB as f64,
        Unit::TIB => bytes as f64 / TIB as f64,
        Unit::PIB => bytes as f64 / PIB as f64,
    };

    let mut str = format!("{:.2}", result)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();

    match unit {
        Unit::B => str.push_str(" B"),
        Unit::KB => str.push_str(" KB"),
        Unit::MB => str.push_str(" MB"),
        Unit::GB => str.push_str(" GB"),
        Unit::TB => str.push_str(" TB"),
        Unit::PB => str.push_str(" PB"),
        Unit::KIB => str.push_str(" KiB"),
        Unit::MIB => str.push_str(" MiB"),
        Unit::GIB => str.push_str(" GiB"),
        Unit::TIB => str.push_str(" TiB"),
        Unit::PIB => str.push_str(" PiB"),
    }

    str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_unit() {
        assert_eq!(parse_size_unit("123").unwrap(), (123_f64, Unit::B));
        assert_eq!(parse_size_unit("12.34").unwrap(), (12.34_f64, Unit::B));
        assert_eq!(parse_size_unit("123B").unwrap(), (123_f64, Unit::B));
        assert_eq!(parse_size_unit("12.34B").unwrap(), (12.34_f64, Unit::B));

        assert_eq!(parse_size_unit("12.34kb").unwrap(), (12.34_f64, Unit::KB));
        assert_eq!(parse_size_unit("12.34kib").unwrap(), (12.34_f64, Unit::KIB));
        assert_eq!(parse_size_unit("12.34KB").unwrap(), (12.34_f64, Unit::KB));
        assert_eq!(parse_size_unit("12.34KiB").unwrap(), (12.34_f64, Unit::KIB));
        
        assert_eq!(parse_size_unit("12.34mb").unwrap(), (12.34_f64, Unit::MB));
        assert_eq!(parse_size_unit("12.34mib").unwrap(), (12.34_f64, Unit::MIB));
        assert_eq!(parse_size_unit("12.34MB").unwrap(), (12.34_f64, Unit::MB));
        assert_eq!(parse_size_unit("12.34MiB").unwrap(), (12.34_f64, Unit::MIB));

        assert_eq!(parse_size_unit("12.34gb").unwrap(), (12.34_f64, Unit::GB));
        assert_eq!(parse_size_unit("12.34gib").unwrap(), (12.34_f64, Unit::GIB));
        assert_eq!(parse_size_unit("12.34GB").unwrap(), (12.34_f64, Unit::GB));
        assert_eq!(parse_size_unit("12.34GiB").unwrap(), (12.34_f64, Unit::GIB));

        assert_eq!(parse_size_unit("12.34tb").unwrap(), (12.34_f64, Unit::TB));
        assert_eq!(parse_size_unit("12.34tib").unwrap(), (12.34_f64, Unit::TIB));
        assert_eq!(parse_size_unit("12.34TB").unwrap(), (12.34_f64, Unit::TB));
        assert_eq!(parse_size_unit("12.34TiB").unwrap(), (12.34_f64, Unit::TIB));

        assert_eq!(parse_size_unit("12.34pb").unwrap(), (12.34_f64, Unit::PB));
        assert_eq!(parse_size_unit("12.34pib").unwrap(), (12.34_f64, Unit::PIB));
        assert_eq!(parse_size_unit("12.34PB").unwrap(), (12.34_f64, Unit::PB));
        assert_eq!(parse_size_unit("12.34PiB").unwrap(), (12.34_f64, Unit::PIB));

        assert_eq!(parse_size_unit("12.34 kb").unwrap(), (12.34_f64, Unit::KB));
        assert_eq!(parse_size_unit("12.34 kib").unwrap(), (12.34_f64, Unit::KIB));
        assert_eq!(parse_size_unit("12.34 KB").unwrap(), (12.34_f64, Unit::KB));
        assert_eq!(parse_size_unit("12.34 KiB").unwrap(), (12.34_f64, Unit::KIB));
        
        assert_eq!(parse_size_unit("12.34 mb").unwrap(), (12.34_f64, Unit::MB));
        assert_eq!(parse_size_unit("12.34 mib").unwrap(), (12.34_f64, Unit::MIB));
        assert_eq!(parse_size_unit("12.34 MB").unwrap(), (12.34_f64, Unit::MB));
        assert_eq!(parse_size_unit("12.34 MiB").unwrap(), (12.34_f64, Unit::MIB));

        assert_eq!(parse_size_unit("12.34 gb").unwrap(), (12.34_f64, Unit::GB));
        assert_eq!(parse_size_unit("12.34 gib").unwrap(), (12.34_f64, Unit::GIB));
        assert_eq!(parse_size_unit("12.34 GB").unwrap(), (12.34_f64, Unit::GB));
        assert_eq!(parse_size_unit("12.34 GiB").unwrap(), (12.34_f64, Unit::GIB));

        assert_eq!(parse_size_unit("12.34 tb").unwrap(), (12.34_f64, Unit::TB));
        assert_eq!(parse_size_unit("12.34 tib").unwrap(), (12.34_f64, Unit::TIB));
        assert_eq!(parse_size_unit("12.34 TB").unwrap(), (12.34_f64, Unit::TB));
        assert_eq!(parse_size_unit("12.34 TiB").unwrap(), (12.34_f64, Unit::TIB));

        assert_eq!(parse_size_unit("12.34 pb").unwrap(), (12.34_f64, Unit::PB));
        assert_eq!(parse_size_unit("12.34 pib").unwrap(), (12.34_f64, Unit::PIB));
        assert_eq!(parse_size_unit("12.34 PB").unwrap(), (12.34_f64, Unit::PB));
        assert_eq!(parse_size_unit("12.34 PiB").unwrap(), (12.34_f64, Unit::PIB));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("123").unwrap(), 123);
        assert_eq!(parse("1.23B").unwrap(), 1);
        assert_eq!(parse("1.23KB").unwrap(), 1_230);
        assert_eq!(parse("1.23MB").unwrap(), 1_230_000);
        assert_eq!(parse("1.23GB").unwrap(), 1_230_000_000);
        assert_eq!(parse("1.23TB").unwrap(), 1_230_000_000_000);
        assert_eq!(parse("1.23PB").unwrap(), 1_230_000_000_000_000);
        assert_eq!(parse("1.23KIB").unwrap(), 1_259);
        assert_eq!(parse("1.23MIB").unwrap(), 1_289_748);
        assert_eq!(parse("1.23GIB").unwrap(), 1_320_702_443);
        assert_eq!(parse("1.23TIB").unwrap(), 1_352_399_302_164);
        assert_eq!(parse("1.23PIB").unwrap(), 1_384_856_885_416_427);
    }

    #[test]
    fn test_parse_to() {
        assert_eq!(parse_to("123", Unit::KB).unwrap(), 0.123);
        assert_eq!(format!("{:.2}", parse_to("1.23KB", Unit::KB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23MB", Unit::MB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23GB", Unit::GB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23TB", Unit::TB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23PB", Unit::PB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23KIB", Unit::KIB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23MIB", Unit::MIB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23GIB", Unit::GIB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23TIB", Unit::TIB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23PIB", Unit::PIB).unwrap()), "1.23");
    }

    #[test]
    fn test_format() {
        assert_eq!(format(123), "123 B");
        assert_eq!(format(1_230), "1.23 KB");
        assert_eq!(format(1_230_000), "1.23 MB");
        assert_eq!(format(1_230_000_000), "1.23 GB");
        assert_eq!(format(1_230_000_000_000), "1.23 TB");
        assert_eq!(format(1_230_000_000_000_000), "1.23 PB");
    }


    #[test]
    fn test_format_to() {
        assert_eq!(format_to(123, Unit::B), "123 B");
        assert_eq!(format_to(1_245, Unit::KB), "1.25 KB");
        assert_eq!(format_to(1_245_000, Unit::MB), "1.25 MB");
        assert_eq!(format_to(1_245_000_000, Unit::GB), "1.25 GB");
        assert_eq!(format_to(1_245_000_000_000, Unit::TB), "1.25 TB");
        assert_eq!(format_to(1_245_000_000_000_000, Unit::PB), "1.25 PB");
        assert_eq!(format_to(1_275, Unit::KIB), "1.25 KiB");
        assert_eq!(format_to(1_306_525, Unit::MIB), "1.25 MiB");
        assert_eq!(format_to(1_337_882_312, Unit::GIB), "1.25 GiB");
        assert_eq!(format_to(1_369_991_488_208, Unit::TIB), "1.25 TiB");
        assert_eq!(format_to(1_402_871_283_925_909, Unit::PIB), "1.25 PiB");

        assert_eq!(format_to(500, Unit::KB), "0.5 KB");
        assert_eq!(format_to(500_000, Unit::MB), "0.5 MB");
        assert_eq!(format_to(500_000_000, Unit::GB), "0.5 GB");
        assert_eq!(format_to(500_000_000_000, Unit::TB), "0.5 TB");
        assert_eq!(format_to(500_000_000_000_000, Unit::PB), "0.5 PB");
        assert_eq!(format_to(512, Unit::KIB), "0.5 KiB");
        assert_eq!(format_to(524_288, Unit::MIB), "0.5 MiB");
        assert_eq!(format_to(536_870_912, Unit::GIB), "0.5 GiB");
        assert_eq!(format_to(549_755_813_888, Unit::TIB), "0.5 TiB");
        assert_eq!(format_to(562_949_953_421_312, Unit::PIB), "0.5 PiB");
    }

    #[test]
    fn test_readme() {
        let input = "1.23 MB";

        // Parse string into bytes
        let bytes: u64 = parse(input).unwrap();
        assert_eq!(bytes, 1_230_000);

        // Format bytes into string
        let bytes_str = format(bytes);
        assert_eq!(&bytes_str, input);

        // Parse to specific unit
        let kb: f64 = parse_to(input, Unit::KB).unwrap();
        assert_eq!(kb, 1_230 as f64);

        // Format to specific unit
        let kb_str = format_to(bytes, Unit::KB);
        assert_eq!(&kb_str, "1230 KB");
    }
}
