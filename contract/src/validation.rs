use soroban_sdk::Env;
use crate::errors::ContractError;

pub fn require_positive_amount(env: &Env, amount: i128) {
    if amount <= 0 {
        env.panic_with_error(ContractError::AmountMustBePositive);
    }
}

pub fn require_positive_interval(env: &Env, interval: u64) {
    if interval == 0 {
        env.panic_with_error(ContractError::IntervalMustBePositive);
    }
}

pub fn require_active_subscription(env: &Env, active: bool) {
    if !active {
        env.panic_with_error(ContractError::SubscriptionInactive);
    }
}

pub fn require_charge_interval_elapsed(env: &Env, now: u64, last_charged: u64, interval: u64) {
    if now < last_charged + interval {
        env.panic_with_error(ContractError::IntervalNotElapsed);
    }
}
