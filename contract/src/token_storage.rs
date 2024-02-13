use crate::storage_types::DataKey;
use soroban_sdk::{Address, Env};

pub fn write_native_token(e: &Env, native_token: &Address) {
    let key = DataKey::NativeToken;
    e.storage().persistent().set(&key, native_token);
}

pub fn read_native_token(e: &Env) -> Address {
    let key = DataKey::NativeToken;
    match e.storage().persistent().get::<DataKey, Address>(&key) {
        Some(native_token) => native_token,
        None => panic!("native token not set"),
    }
}

pub fn read_series(e: &Env) -> u128 {
    let key = DataKey::Series;
    e.storage()
        .persistent()
        .get::<DataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn increment_series(e: &Env) -> u128 {
    let key = DataKey::Series;
    let next_supply = read_series(e) + 1;
    e.storage().persistent().set(&key, &next_supply);
    next_supply
}
