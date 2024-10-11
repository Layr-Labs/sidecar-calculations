use bigdecimal::{BigDecimal, Context};
use std::str::FromStr;
use serde_json::Value;

pub fn nile_staker_token_rewards(sp: &str, tpd: &str) -> BigDecimal {
    let staker_proportion = BigDecimal::from_str(sp).unwrap();
    let tokens_per_day = BigDecimal::from_str(tpd).unwrap();

    let final_prec = core::num::NonZeroU64::try_from(15).unwrap();
    (staker_proportion * tokens_per_day).with_precision_round(final_prec, bigdecimal::RoundingMode::HalfUp)
}
