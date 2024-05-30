use crate::{bump::extend_persistent, storage_types::DataKey};
use crate::soroban_sdk::{Address, Env};

pub fn read_creator(e: &Env, id: u128) -> Address {
    let key = DataKey::Creator(id);
    match e.storage().persistent().get(&key) {
        Some(owner) => owner,
        None => panic!("creator not found"),
    }
}

pub fn write_creator(e: &Env, id: u128, owner: &Address) {
    let key = DataKey::Creator(id);
    e.storage().persistent().set(&key, owner);
}

pub fn expand_creator_ttl(e: &Env, id: u128) {
    let key = DataKey::Creator(id);
    extend_persistent(e, &key);
}

pub fn read_creator_curved(e: &Env, id: u128) -> u128 {
    let key = DataKey::CreatorCurved(id);
    e.storage().persistent().get(&key).unwrap_or(0)
}

pub fn write_creator_curved(e: &Env, id: u128, curved: u128) {
    let key = DataKey::CreatorCurved(id);
    e.storage().persistent().set(&key, &curved);
}

pub fn expand_creator_curved_ttl(e: &Env, id: u128) {
    let key = DataKey::CreatorCurved(id);
    extend_persistent(e, &key);
}
