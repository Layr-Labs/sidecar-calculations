use bigdecimal::BigDecimal;
use std::str::FromStr;

/// Subtract two numbers, bigly
pub fn subtract_big(a_str: &str, b_str: &str) -> String {
    let a = BigDecimal::from_str(a_str).unwrap();
    let b = BigDecimal::from_str(b_str).unwrap();

    (a - b).to_string()
}

/// Add two numbers, bigly
pub fn add_big(a_str: &str, b_str: &str) -> String {
    let a = BigDecimal::from_str(a_str).unwrap();
    let b = BigDecimal::from_str(b_str).unwrap();

    (a + b).to_string()
}
