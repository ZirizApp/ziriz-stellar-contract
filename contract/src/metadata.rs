use crate::storage_types::{DataKey, UserDataKey};
use soroban_sdk::{Env, String, Vec, Address};
use crate::data_type::Metadata;

pub fn read_name(e: &Env) -> String {
    let key = DataKey::Name;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_name(e: &Env, name: &String) {
    let key = DataKey::Name;
    e.storage().instance().set(&key, name);
}

pub fn read_symbol(e: &Env) -> String {
    let key = DataKey::Symbol;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_symbol(e: &Env, symbol: &String) {
    let key = DataKey::Symbol;
    e.storage().instance().set(&key, symbol);
}

pub fn read_metadata(e: &Env, id: u128) -> Metadata {
    let key = DataKey::Metadata(id);
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_metadata(e: &Env, id: u128, metadata: &Metadata) {
    let key = DataKey::Metadata(id);
    e.storage().persistent().set(&key, metadata);
}

pub fn map_token_to_series(e: &Env, id: u128, series_id: &u128) {
    let key = DataKey::Token(id);
    e.storage().instance().set(&key, series_id);
}

pub fn get_series_id(e: &Env, id: u128) -> u128 {
    let key = DataKey::Token(id);
    e.storage().instance().get(&key).unwrap()
}

pub fn read_owned_tokens(e: &Env, owner: &Address) -> Vec<u128> {
    let key = UserDataKey::OwnedTokens(owner.clone());
    let tokens = e.storage().persistent().get(&key);
    match tokens {
        Some(tokens) => tokens,
        None => Vec::new(e),
    }
}

pub fn map_token_to_owner(e: &Env, id: u128, owner: &Address) {
    let key = UserDataKey::OwnedTokens(owner.clone());
    let mut tokens = read_owned_tokens(&e, owner);
    tokens.push_back(id);
    e.storage().persistent().set(&key, &tokens);
}