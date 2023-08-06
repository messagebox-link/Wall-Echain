use bigdecimal::{BigDecimal, Num, num_bigint::BigInt};
use chrono::{Utc, TimeZone};
use regex::Regex;
use std::num::ParseIntError;

pub fn hex2num(hex: &str) -> Result<i128, ParseIntError> {
    if hex=="0x"{
        return Ok(0)
    }
    let re = Regex::new("0x[0-9a-f]*").unwrap();
    let t = re.captures(hex).unwrap();
    let s = &t[0];
    let without_prefix = s.trim_start_matches("0x");
    return i128::from_str_radix(without_prefix, 16);
}

pub fn num2hex(num: i128) -> String {
    format!("0x{:X}", num).to_ascii_lowercase()
}

// 获取UTC时间
pub fn hex2utc(timestamp: &str) -> String {
    let timestamp = hex2num(timestamp).unwrap();
    let utc = Utc.timestamp_opt(timestamp.try_into().unwrap(), 0).unwrap();
    let utc = utc.format("%Y-%m-%d %H:%M:%S").to_string();
    utc
}

pub fn hex2decimal(hex: &str, decimal: u32) -> BigDecimal {
    let bigint = BigInt::from_str_radix(&hex[2..], 16).unwrap();
    let bigdecimal = BigDecimal::from(bigint)/BigDecimal::from(10_i64.pow(decimal));
    bigdecimal
}

#[test]
fn test() {
    let num: i128 = 12;
    let hex = num2hex(num);
    println!("{} num2hex => {:?}", num, hex);

    let hex = "0x".to_string();
    let num = hex2num(&hex).unwrap();
    println!("{} hex2num => {:?}", hex, num);

    let hex = "0x6246df2f".to_string();
    let utc = hex2utc(&hex);
    println!("{} hex2utc => {:?}", hex, utc);

    let hex = "0x12".to_string();
    let decimal = 6;
    let big_decimal = hex2decimal(&hex, decimal);
    println!("{} hex2decimal => {:?}", hex, big_decimal);
}
