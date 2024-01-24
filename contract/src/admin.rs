use soroban_sdk::{Address, BytesN, Env};
use crate::storage_types::{DataKey};

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_admin(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_admin(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

pub fn read_wasm(e: &Env)->BytesN<32>{
  let key = DataKey::Wasm;
  e.storage().instance().get(&key).unwrap()
}

pub fn write_wasm(e: &Env, hash: &BytesN<32>) {
  let key = DataKey::Wasm;
  e.storage().instance().set(&key, hash);
}