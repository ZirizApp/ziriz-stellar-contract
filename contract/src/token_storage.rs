use crate::storage_types::{DataKey, UserDataKey};
use soroban_sdk::{Address, Env};

pub fn write_native_token(e: &Env, native_token: &Address) {
    let key = DataKey::NativeToken;
    e.storage().instance().set(&key, native_token);
}

pub fn read_native_token(e: &Env) -> Address {
    let key = DataKey::NativeToken;
    match e.storage().instance().get::<DataKey, Address>(&key) {
        Some(native_token) => native_token,
        None => panic!("native token not set"),
    }
}

pub fn read_series(e: &Env) -> u128 {
    let key = DataKey::Series;
    e.storage()
        .instance()
        .get::<DataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn increment_series(e: &Env) -> u128 {
    let key = DataKey::Series;
    let next_supply = read_series(e) + 1;
    e.storage().instance().set(&key, &next_supply);
    next_supply
}


pub fn read_series_balance(e: &Env, account: &Address, id: u128) -> u128 {
    let key = UserDataKey::SeriesBalance(account.clone(), id);
    e.storage()
        .persistent()
        .get::<UserDataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn increment_series_balance(e: &Env, account: &Address, id: u128) {
    let key = UserDataKey::SeriesBalance(account.clone(), id);
    e.storage()
        .persistent()
        .set(&key, &(read_series_balance(e, account, id) + 1));
}
