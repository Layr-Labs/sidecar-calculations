use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

/// Subtract two numbers, bigly
pub fn subtract_big(a_str: &str, b_str: &str) -> String {
    let a = BigDecimal::from_str(a_str).unwrap();
    let b = BigDecimal::from_str(b_str).unwrap();

    (a - b).to_string()
}

#[no_mangle]
pub extern "C" fn subtract_big_c(a: *const c_char, b: *const c_char) -> *mut c_char {
    let a_str = unsafe {
        assert!(!a.is_null());
        CStr::from_ptr(a).to_str().unwrap()
    };
    let b_str = unsafe {
        assert!(!b.is_null());
        CStr::from_ptr(b).to_str().unwrap()
    };

    let result = subtract_big(a_str, b_str);

    CString::new(result).unwrap().into_raw()
}

/// Add two numbers, bigly
pub fn add_big(a_str: &str, b_str: &str) -> String {
    let a = BigDecimal::from_str(a_str).unwrap();
    let b = BigDecimal::from_str(b_str).unwrap();

    (a + b).to_string()
}

#[no_mangle]
pub extern "C" fn add_big_c(a: *const c_char, b: *const c_char) -> *mut c_char {
    let a_str = unsafe {
        assert!(!a.is_null());
        CStr::from_ptr(a).to_str().unwrap()
    };
    let b_str = unsafe {
        assert!(!b.is_null());
        CStr::from_ptr(b).to_str().unwrap()
    };

    let result = add_big(a_str, b_str);

    CString::new(result).unwrap().into_raw()
}

pub fn big_gt(a_str: &str, b_str: &str) -> bool {
    let a = BigDecimal::from_str(a_str).unwrap();
    let b = BigDecimal::from_str(b_str).unwrap();

    a > b
}

#[no_mangle]
pub extern "C" fn big_gt_c(a: *const c_char, b: *const c_char) -> i8 {
    let a_str = unsafe {
        assert!(!a.is_null());
        CStr::from_ptr(a).to_str().unwrap()
    };
    let b_str = unsafe {
        assert!(!b.is_null());
        CStr::from_ptr(b).to_str().unwrap()
    };

    match big_gt(a_str, b_str) {
        true => 1,
        false => 0,
    }
}

pub fn numeric_multiply(multiplier_str: &str, shares_str: &str) -> String {
    let a = BigDecimal::from_str(multiplier_str).unwrap();
    let b = BigDecimal::from_str(shares_str).unwrap();

    (a * b).to_string()
}

#[no_mangle]
pub extern "C" fn numeric_multiply_c(a: *const c_char, b: *const c_char) -> *mut c_char {
    let a_str = unsafe {
        assert!(!a.is_null());
        CStr::from_ptr(a).to_str().unwrap()
    };
    let b_str = unsafe {
        assert!(!b.is_null());
        CStr::from_ptr(b).to_str().unwrap()
    };

    let result = numeric_multiply(a_str, b_str);

    CString::new(result).unwrap().into_raw()
}
