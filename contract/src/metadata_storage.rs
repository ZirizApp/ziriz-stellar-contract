use crate::bump::extend_persistent;
use crate::data_type::Metadata;
use crate::storage_types::DataKey;
use soroban_sdk::Env;

pub fn read_metadata(e: &Env, id: u128) -> Metadata {
    let key = DataKey::Metadata(id);
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_metadata(e: &Env, id: u128, metadata: &Metadata) {
    let key = DataKey::Metadata(id);
    e.storage().persistent().set(&key, metadata);
}

pub fn expand_metadata_ttl(e: &Env, id: u128) {
    let key = DataKey::Metadata(id);
    extend_persistent(e, &key);
}
