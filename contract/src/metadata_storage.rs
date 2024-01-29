use crate::data_type::Metadata;
use crate::storage_types::DataKey;
use soroban_sdk::{Env, String};

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
