use core::ops::Mul;

use crate::{
    series::{read_fan_cut, read_series_sales},
    storage_types::UserDataKey,
};
use soroban_sdk::{Address, Env, Vec};

pub fn read_series_order(env: &Env, account: &Address, id: u128) -> Vec<u128> {
    let key = UserDataKey::OwnedSeriesOrder(account.clone(), id);
    match env.storage().persistent().get(&key) {
        Some(tokens) => tokens,
        None => Vec::new(env),
    }
}

pub fn map_series_order(env: &Env, account: &Address, id: u128, order_id: u128) {
    let key = UserDataKey::OwnedSeriesOrder(account.clone(), id);
    let mut tokens = match env.storage().persistent().get(&key) {
        Some(tokens) => tokens,
        None => Vec::new(env),
    };
    tokens.push_back(order_id);
    env.storage().persistent().set(&key, &tokens);
}

pub fn read_last_whitdrawn(env: &Env, account: &Address, id: u128) -> u128 {
    let key = UserDataKey::LastClaim(account.clone(), id);
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn write_last_whitdrawn(env: &Env, account: &Address, id: u128, last_whitdrawn: u128) {
    let key = UserDataKey::LastClaim(account.clone(), id);
    env.storage().persistent().set(&key, &last_whitdrawn);
}

pub fn get_share_balance(env: &Env, account: &Address, id: u128) -> u128 {
    let mut share: u128 = 0;
    let orders = read_series_order(env, account, id);
    let current_sales = read_series_sales(env, id);
    let last_whitdrawn = read_last_whitdrawn(env, account, id);
    if last_whitdrawn < current_sales {
        for order in orders.iter() {
            let fan_cut = read_fan_cut(env, id, order + 1); // next fan cut after this order
            share += fan_cut.mul(current_sales - last_whitdrawn.max(order)); // your fan cut * number of sales since last whitdrawn
        }
    }
    share
}
