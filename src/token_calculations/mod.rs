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

/// Calculate the staker proportion
///
/// original query: FLOOR((staker_weight / total_weight) * 1000000000000000) / 1000000000000000
pub fn staker_proportion(staker_weight: &str, total_staker_weight: &str) -> String {
    let staker_weight = BigDecimal::from_str(staker_weight).unwrap();
    let total_staker_weight = BigDecimal::from_str(total_staker_weight).unwrap();

    let final_prec = core::num::NonZeroU64::try_from(38).unwrap();
    let result = (staker_weight / total_staker_weight).with_precision_round(final_prec, bigdecimal::RoundingMode::HalfEven);

    let mut pre_result = result * BigDecimal::from_str("1000000000000000").unwrap();

    pre_result = pre_result.with_scale_round(0, bigdecimal::RoundingMode::Floor);

    let final_result = pre_result / BigDecimal::from_str("1000000000000000").unwrap();

    final_result.to_string()
}

#[no_mangle]
pub extern "C" fn staker_proportion_c(staker_weight: *const c_char, total_staker_weight: *const c_char) -> *mut c_char {
    let staker_weight_str = unsafe {
        assert!(!staker_weight.is_null());
        CStr::from_ptr(staker_weight).to_str().unwrap()
    };
    let total_staker_weight_str = unsafe {
        assert!(!total_staker_weight.is_null());
        CStr::from_ptr(total_staker_weight).to_str().unwrap()
    };

    let result = staker_proportion(staker_weight_str, total_staker_weight_str);

    CString::new(result).unwrap().into_raw()
}

pub fn base_tokens_per_day(amount_str: &str, duration_str: &str) -> BigDecimal {
    let amount = BigInt::from_str(amount_str).unwrap();

    // duration and days_seconds will always be integers, so treat them as such
    let day_seconds = BigInt::from(86400);
    let duration = BigInt::from_str(duration_str).unwrap();


    // when dividing duration by 86400, the original Postgres query is performing integer division,
    // so we'll do the same here
    let duration_days = duration / day_seconds;

    // When converting to a numeric type and determining the scale of the resulting number,
    // postgres uses the formula: scale(numeric1) + scale(numeric2) + 4
    //
    // In this case we'll have: scale(amount) + scale(duration/86400) + 4
    //
    // * duration/86400 is performing integer division, but it's scale can be up to 4 decimal places
    // * amount is a BigInteger, so it's scale is 0
    // In total we'll have up to 8 decimal places
    let mut tpd = BigDecimal::from(amount) / BigDecimal::from(duration_days);
    let pre_round_left_digits = tpd.to_bigint().unwrap().to_string().len() as i64;

    // Postgres seems to follow this model of determining how to balance
    // the precision of the number. This is a heuristic, so it may not be perfect.
    let prec: i64;
    if tpd < BigDecimal::from_str("1").unwrap() {
        prec = 20;
    } else if pre_round_left_digits <= 4 {
        prec = 16;
    } else if pre_round_left_digits <= 8 {
        prec = 12;
    } else if pre_round_left_digits <= 12 {
        prec = 8;
    } else {
        prec = 4;
    }
    tpd = tpd.with_scale_round(prec, RoundingMode::HalfDown);


    let int_tpd = tpd.to_bigint().unwrap();
    let tpd_left_digits = int_tpd.to_string().len() as i64;

    let max_left_digits = 16;

    // Another heuristic: if the number of digits to the left of the decimal is greater than 16,
    // truncate the number to 0 decimal places
    if tpd_left_digits > max_left_digits {
        tpd = tpd.with_scale_round(0, RoundingMode::HalfUp);
    } else if tpd.is_integer() {
        tpd = tpd.with_scale_round(0, RoundingMode::Floor);
    }
    tpd
}

/// Calculate the operator weight by multiplying the operator shares by the multiplier.
/// This function is entirely powered by heuristics. The corresponding test function takes
/// inputs and outputs from mainnet, testnet and preprod for the for the period of time
/// this function was active to ensure its correctness.
///
/// Original query: floor(amount/(duration/86400)) as tokens_per_day
pub fn tokens_per_day(amount_str: &str, duration_str: &str) -> String {
    let tpd = base_tokens_per_day(amount_str, duration_str);
    // -----
    // The above code correctly calculates the first tokens per day calc before rounding to 15 sigfig
    // e.g. (amount/(duration / 86400))
    // -----

    // This works for mainnet...and is janky AS FUCK
    //
    // This is the most magical number ever.
    // Turns out, postgres is using some extra precision somewhere where the result of
    // ((POW(10, 15) - 1)/(POW(10, 15))) is not exactly 0.999999999999999
    //
    // This was determined by running: select 178571428571428384::numeric / 178571428571428571::numeric
    // where the left side is the tokens_per_day and right side is tokens_per_day after trying
    // to round to 15 sigfigs
    let multiplier = BigDecimal::from_str("0.9999999999999989528").unwrap();

    let mut result = &tpd * multiplier;

    let result_str = result.to_string();
    let result_int_part = result_str.split_once(".").unwrap().0.to_string();

    // Handle some cases we see in testnet
    if result_int_part.len() < 18 {
        if result.is_integer() {
            return result_str;
        }
        let result_int = result.to_bigint().unwrap().to_string();
        if result_int.len() <= 12 {
            return result.with_scale_round(8, RoundingMode::Up).to_string()
        }

        return result_str;
    }

    // Handle cases for mainnet
    result = result.with_scale(0);
    if result.to_string().len() <= 18 {
        return result.to_string();
    }

    let left = &result_int_part[0..16];
    let right_count = result_int_part.len() - 16;

    return format!("{}{}", left, "0".repeat(right_count));
}

pub fn old_tokens_per_day(amount: &str, duration: &str) -> String {
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
    let tpd = base_tokens_per_day(amount, duration);
    tpd.with_scale_round(0, RoundingMode::Floor).to_string()
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
