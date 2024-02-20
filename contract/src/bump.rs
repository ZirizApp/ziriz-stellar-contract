use soroban_sdk::{Address, Env};

use crate::{
    metadata_storage::expand_metadata_ttl,
    owner_storage::{expand_creator_curved_ttl, expand_creator_ttl},
    series_storage::{
        expand_fan_base_price_ttl, expand_fan_cut_ttl, expand_fan_decay_rate_ttl,
        expand_series_price_ttl, expand_series_sales_ttl, expand_sum_fan_cut_ttl,
    },
    share_storage::{expand_last_withdrawn_ttl, expand_series_order_ttl, read_series_order},
    storage_types::{DataKey, UserDataKey},
};

pub const DAY_IN_LEDGERS: u32 = 17280;

pub const PERSISTENT_BUMP_CONSTANT: u32 = DAY_IN_LEDGERS * 180;
pub const PERSISTENT_BUMP_CONSTANT_THRESHOLD: u32 = DAY_IN_LEDGERS * 90;

pub fn extend_instance(env: &Env) {
    extend_persistent(env, &DataKey::Admin);
    extend_persistent(env, &DataKey::Wasm);
    extend_persistent(env, &DataKey::Series);
    extend_persistent(env, &DataKey::NativeToken);
}

pub fn extend_persistent(env: &Env, key: &DataKey) {
    if env.storage().persistent().has(key) {
        env.storage().persistent().extend_ttl(
            key,
            PERSISTENT_BUMP_CONSTANT_THRESHOLD,
            PERSISTENT_BUMP_CONSTANT,
        );
    }
}

pub fn extend_user_persistent(env: &Env, key: &UserDataKey) {
    if env.storage().persistent().has(key) {
        env.storage().persistent().extend_ttl(
            key,
            PERSISTENT_BUMP_CONSTANT_THRESHOLD,
            PERSISTENT_BUMP_CONSTANT,
        );
    }
}

pub fn extend_series(env: &Env, series_id: u128) {
    expand_metadata_ttl(env, series_id);
    expand_creator_ttl(env, series_id);
    expand_creator_curved_ttl(env, series_id);
    expand_series_sales_ttl(env, series_id);
    expand_series_price_ttl(env, series_id);
    expand_fan_base_price_ttl(env, series_id);
    expand_fan_decay_rate_ttl(env, series_id);
    expand_sum_fan_cut_ttl(env, series_id);
}

pub fn extend_account_series_balance(env: &Env, account: &Address, series_id: u128) {
    expand_last_withdrawn_ttl(env, account, series_id);
    expand_series_order_ttl(env, account, series_id);
    let orders = read_series_order(env, account, series_id);
    for order in orders.iter() {
        expand_fan_cut_ttl(env, series_id, order + 1);
    }
}
