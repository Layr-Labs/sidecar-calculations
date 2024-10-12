use bigdecimal::{BigDecimal};
use std::str::FromStr;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

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
