use bigdecimal::{BigDecimal};
use std::str::FromStr;

pub fn amazon_staker_token_rewards(sp: &str, tpd: &str) -> String {
    let staker_proportion = BigDecimal::from_str(sp).unwrap();
    let tokens_per_day = BigDecimal::from_str(tpd).unwrap();

    let final_prec = core::num::NonZeroU64::try_from(15).unwrap();
    (staker_proportion * tokens_per_day).with_precision_round(final_prec, bigdecimal::RoundingMode::HalfUp).to_string()
}

pub fn amazon_operator_token_rewards(tsot: &str) -> String {
    let total_staker_operator_tokens = BigDecimal::from_str(tsot).unwrap();
    let operator_commission = BigDecimal::from_str(".10").unwrap();

    let result = operator_commission * total_staker_operator_tokens;

    result.with_scale_round(0, bigdecimal::RoundingMode::HalfUp).to_string()
}
