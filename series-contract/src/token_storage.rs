use crate::storage_types::{DataKey, UserDataKey};
use soroban_sdk::{Address, Env};

pub fn read_supply(e: &Env) -> u128 {
    let key = DataKey::Supply;
    e.storage()
        .instance()
        .get::<DataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn increment_supply(e: &Env) -> u128 {
    let key = DataKey::Supply;
    let next_supply = read_supply(e) + 1;
    e.storage().instance().set(&key, &next_supply);
    next_supply
}

pub fn read_balance(e: &Env, id: &Address) -> u128 {
    let key = UserDataKey::Balance(id.clone());
    e.storage()
        .persistent()
        .get::<UserDataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn increment_balance(e: &Env, id: &Address) {
    let key = UserDataKey::Balance(id.clone());
    e.storage()
        .persistent()
        .set(&key, &(read_balance(e, id) + 1));
}

