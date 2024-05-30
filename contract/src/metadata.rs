use crate::storage_types::DataKey;
use crate::soroban_sdk::{self, Env, String};
use crate::data_type::Metadata;

pub fn read_metadata(e: &Env, id: u128) -> Metadata {
    let key = DataKey::Metadata(id);
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_metadata(e: &Env, id: u128, metadata: &Metadata) {
    let key = DataKey::Metadata(id);
    e.storage().persistent().set(&key, metadata);
}
