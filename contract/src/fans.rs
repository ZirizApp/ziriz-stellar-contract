
use soroban_sdk::{Env, Address, Vec};
use crate::storage_types::DataKey;

pub fn read_fans(env: &Env, id: u128) -> Vec<Address> {
    let key = DataKey::Fans(id);
    let fans = env.storage().persistent().get(&key);
    match fans {
        Some(fans) => fans,
        None => Vec::new(env),
    }
}

pub fn add_fans(env: &Env, id: u128, fan: &Address) {
    let key = DataKey::Fans(id);
    let mut fans = read_fans(&env, id);
    fans.push_back(fan.clone());
    env.storage().persistent().set(&key, &fans);
}

pub fn is_already_fan(env: &Env, id: u128, fan: &Address) -> bool {
    let fans = read_fans(&env, id);
    fans.contains(fan)
}