// use bigdecimal::{BigDecimal};
use std::str::FromStr;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use num::bigint::*;
use bigdecimal::*;

/// Calculate the staker token rewards by multiplying the staker proportion by the tokens per day.
pub fn staker_token_rewards(sp: &str, tpd: &str) -> String {
    let staker_proportion = BigDecimal::from_str(sp).unwrap();
    let tokens_per_day = BigDecimal::from_str(tpd).unwrap();

    let final_prec = core::num::NonZeroU64::try_from(38).unwrap();
    let result = (staker_proportion * tokens_per_day).with_precision_round(final_prec, bigdecimal::RoundingMode::HalfEven);

    result.with_scale_round(0, bigdecimal::RoundingMode::Down).to_string()
}

#[no_mangle]
pub extern "C" fn staker_token_rewards_c(sp: *const c_char, tpd: *const c_char) -> *mut c_char {
    let sp_str = unsafe {
        assert!(!sp.is_null());
        CStr::from_ptr(sp).to_str().unwrap()
    };
    let tpd_str = unsafe {
        assert!(!tpd.is_null());
        CStr::from_ptr(tpd).to_str().unwrap()
    };

    let result = staker_token_rewards(sp_str, tpd_str);

    CString::new(result).unwrap().into_raw()
}

/// Calculate the operator token rewards by multiplying the total staker operator tokens by the
/// operator commission, which is currently fixed at 10%
pub fn operator_token_rewards(tsot: &str) -> String {
    staker_token_rewards(tsot, ".10")
}

#[no_mangle]
pub extern "C" fn operator_token_rewards_c(tsot: *const c_char) -> *mut c_char {
    let tsot_str = unsafe {
        assert!(!tsot.is_null());
        CStr::from_ptr(tsot).to_str().unwrap()
    };

    let result = operator_token_rewards(tsot_str);

    CString::new(result).unwrap().into_raw()
}

/// Calculate the staker weight by multiplying the staker shares by the multiplier.
pub fn staker_weight(mul: &str, shares: &str) -> String {
    let multiplier = BigDecimal::from_str(mul).unwrap();
    let staker_shares = BigDecimal::from_str(shares).unwrap();

    let final_prec = core::num::NonZeroU64::try_from(38).unwrap();
    let result = (multiplier * staker_shares).with_precision_round(final_prec, bigdecimal::RoundingMode::HalfEven);

    result.with_scale_round(0, bigdecimal::RoundingMode::Down).to_string()
}

#[no_mangle]
pub extern "C" fn staker_weight_c(mul: *const c_char, shares: *const c_char) -> *mut c_char {
    let mul_str = unsafe {
        assert!(!mul.is_null());
        CStr::from_ptr(mul).to_str().unwrap()
    };
    let shares_str = unsafe {
        assert!(!shares.is_null());
        CStr::from_ptr(shares).to_str().unwrap()
    };

    let result = staker_weight(mul_str, shares_str);

    CString::new(result).unwrap().into_raw()
}

pub fn tokens_per_day(amount: &str, duration: &str) -> String {
    let amount_d = BigDecimal::from_str(amount).unwrap();
    let duration_d = duration.parse::<i64>().unwrap();

    let mut per_day: BigDecimal = (BigDecimal::from(duration_d) / 86400);
    if per_day.is_integer() {
        per_day = per_day.with_scale_round(0, RoundingMode::Down);
    } else {
        per_day = per_day.with_scale(28);
    }
    println!("per day:  {}", per_day);
    let mut tpd = amount_d / per_day;
    println!("tpd:      {}", tpd);

    let mut scaled = false;
    if tpd < BigDecimal::from(1) {
        scaled = true;
        tpd = tpd.with_scale_round(20, bigdecimal::RoundingMode::HalfEven);
    }
    println!("tpd 2:    {} (scaled {})", tpd, scaled);

    // strip any trailing 0s
    //tpd = tpd.normalized();

    let mut scale = 16;
    if tpd < BigDecimal::from(1) {
        scale = 20;
    }
    println!("Pre scale digits: {}", tpd.digits());
    let mut scaled_tpd = tpd.with_scale_round(scale, RoundingMode::HalfEven);
    println!("scaled_tpd: {}", scaled_tpd);
    println!("Digits: {}", scaled_tpd.digits());


    if scaled_tpd.is_integer() {
        return scaled_tpd.with_scale_round(0, RoundingMode::HalfEven).to_string();
    }
    // this kinda works
    if scaled_tpd >= BigDecimal::from(1) {
        let b = scaled_tpd.to_bigint().unwrap();
        let left_side_digits = b.to_string().len();

        if left_side_digits >= 18 {
            println!("Truncating....");
            return scaled_tpd.with_scale_round(0, RoundingMode::HalfEven).to_string();
        }

        
        let mut decimal_digits: i64 = scaled_tpd.digits() as i64 - left_side_digits as i64;
        if decimal_digits > 12 {
            decimal_digits = 12;
        }

        let total_digits = left_side_digits as i64 + decimal_digits;

        println!("Decimal digits: {}", decimal_digits);
        scaled_tpd = scaled_tpd.with_scale_round(decimal_digits, RoundingMode::HalfEven);
    }


    scaled_tpd.to_string()
}

#[no_mangle]
pub extern "C" fn tokens_per_day_c(amount: *const c_char, duration: *const c_char) -> *mut c_char {
    let amount_str = unsafe {
        assert!(!amount.is_null());
        CStr::from_ptr(amount).to_str().unwrap()
    };
    let duration_str = unsafe {
        assert!(!duration.is_null());
        CStr::from_ptr(duration).to_str().unwrap()
    };

    let result = tokens_per_day(amount_str, duration_str);

    CString::new(result).unwrap().into_raw()
}

pub fn tokens_per_day_decimal(amount: &str, duration: &str) -> String {
    let amount = BigDecimal::from_str(amount).unwrap();
    let duration = BigDecimal::from_str(duration).unwrap();

    let per_day = duration / BigDecimal::from_str("86400").unwrap();

    let tpd = (amount / per_day).with_prec(22);

    tpd.to_string()
}

#[no_mangle]
pub extern "C" fn tokens_per_day_decimal_c(amount: *const c_char, duration: *const c_char) -> *mut c_char {
    let amount_str = unsafe {
        assert!(!amount.is_null());
        CStr::from_ptr(amount).to_str().unwrap()
    };
    let duration_str = unsafe {
        assert!(!duration.is_null());
        CStr::from_ptr(duration).to_str().unwrap()
    };

    let result = tokens_per_day_decimal(amount_str, duration_str);

    CString::new(result).unwrap().into_raw()
}
