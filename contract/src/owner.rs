use soroban_sdk::{Address, Env};
use crate::storage_types::{UserDataKey};

pub fn read_creator(e: &Env, id: u128) -> Address {
    let key = UserDataKey::Creator(id);
    match e.storage().persistent().get(&key) {
        Some(owner) => owner,
        None => panic!("creator not found"),
    }
}

pub fn write_creator(e: &Env, id: u128, owner: &Address) {
    let key = UserDataKey::Creator(id);
    e.storage().persistent().set(&key, owner);
}

pub fn read_token_owner(e: &Env, id: u128) -> Address {
    let key = UserDataKey::TokenOwner(id);
    match e.storage().persistent().get(&key) {
        Some(owner) => owner,
        None => panic!("invalid token id"),
    }
}

pub fn write_token_owner(e: &Env, id: u128, owner: &Address) {
    let key = UserDataKey::TokenOwner(id);
    e.storage().persistent().set(&key, owner);
}