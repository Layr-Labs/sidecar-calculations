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

/// Calculate the operator weight by multiplying the operator shares by the multiplier.
/// This function is entirely powered by heuristics. The corresponding test function takes
/// inputs and outputs from mainnet, testnet and preprod for the for the period of time
/// this function was active to ensure its correctness.
///
/// Original query: floor(amount/(duration/86400)) as tokens_per_day
pub fn tokens_per_day(amount: &str, duration: &str) -> String {
    let amount_d = BigDecimal::from_str(amount).unwrap();
    let duration_d = duration.parse::<i64>().unwrap();

    let mut per_day: BigDecimal = BigDecimal::from(duration_d) / 86400;

    // If its an integer, strip off all the useless decimal 0s
    if per_day.is_integer() {
        per_day = per_day.with_scale_round(0, RoundingMode::Down);
    } else {
        per_day = per_day.with_scale(28);
    }
    let mut tpd = amount_d / per_day;

    // If its less than 1, round to 20 decimal places, because that totally makes sense /s
    if tpd < BigDecimal::from(1) {
        tpd = tpd.with_scale_round(20, bigdecimal::RoundingMode::HalfEven);
    }
    let mut scale = 16;
    if tpd < BigDecimal::from(1) {
        scale = 20;
    }
    let mut scaled_tpd = tpd.with_scale_round(scale, RoundingMode::HalfEven);


    // If its an integer, just return it
    if scaled_tpd.is_integer() {
        return scaled_tpd.with_scale_round(0, RoundingMode::HalfEven).to_string();
    }

    // God this is so ugly but it works...
    if scaled_tpd >= BigDecimal::from(1) {
        let b = scaled_tpd.to_bigint().unwrap();
        let left_side_digits = b.to_string().len() as i64;

        // heuristically, this seems to be the parameters postgres uses.
        let max_left_digits = 16;
        let max_right_digits = 12;
        let max_total_digits = 20;

        // For really small left whole values, we can make the decimal precision a little bigger,
        // all because postgres says so...
        let exception_max_right_digits = 16;


        // If the left side is too big, just truncate it and return
        if left_side_digits > max_left_digits {
            return scaled_tpd.with_scale_round(0, RoundingMode::HalfEven).to_string();
        }

        let mut decimal_digits: i64 = max_total_digits - left_side_digits;
        // for small left side numbers, let the decimal be a little bigger
        // because who the hell knows why...thats what postgres does for some reason...
        if left_side_digits < 4 {
            if decimal_digits > exception_max_right_digits {
                decimal_digits = exception_max_right_digits;
            }
        } else {
            // Lol guess the number is too big so we have to rely on our max values from above
            if decimal_digits > max_right_digits {
                decimal_digits = max_right_digits;
            }
        }

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

/// Calculate tokens per_day_decimal which isnt actually a decimal since we're technically using a
/// floor function on the result, but thats the column name in the database so we're stuck with it.
///
/// Original query: floor(tokens_per_day) as tokens_per_day_decimal
pub fn tokens_per_day_decimal(amount: &str, duration: &str) -> String {
    let tpd = BigDecimal::from_str(tokens_per_day(amount, duration).as_str()).unwrap();
    return tpd.with_scale_round(0, RoundingMode::Floor).to_string();
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
