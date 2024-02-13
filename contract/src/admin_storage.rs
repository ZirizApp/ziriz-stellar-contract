use crate::storage_types::DataKey;
use soroban_sdk::{Address, BytesN, Env};

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().persistent().has(&key)
}

pub fn read_admin(e: &Env) -> Address {
    let key: DataKey = DataKey::Admin;
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_admin(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().persistent().set(&key, id);
}

pub fn read_wasm(e: &Env) -> BytesN<32> {
    let key = DataKey::Wasm;
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_wasm(e: &Env, hash: &BytesN<32>) {
    let key = DataKey::Wasm;
    e.storage().persistent().set(&key, hash);
}
