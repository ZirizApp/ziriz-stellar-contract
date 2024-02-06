use soroban_sdk::Env;

use crate::storage_types::{DataKey, UserDataKey};

pub const DAY_IN_LEDGERS: u32 = 17280;
pub const INSTANCE_BUMP_CONSTANT: u32 = DAY_IN_LEDGERS * 30;
pub const INSTANCE_BUMP_CONSTANT_THRESHOLD: u32 = DAY_IN_LEDGERS * 14;

pub const PERSISTENT_BUMP_CONSTANT: u32 = DAY_IN_LEDGERS * 30;
pub const PERSISTENT_BUMP_CONSTANT_THRESHOLD: u32 = DAY_IN_LEDGERS * 14;

pub fn extend_instance(env: &Env) {
    env.storage().instance().extend_ttl(INSTANCE_BUMP_CONSTANT_THRESHOLD, INSTANCE_BUMP_CONSTANT);
}

pub fn extend_user_persistent(env: &Env, key: &UserDataKey) {
  if env.storage().persistent().has(key) {
      env.storage().persistent().extend_ttl(key, PERSISTENT_BUMP_CONSTANT_THRESHOLD, PERSISTENT_BUMP_CONSTANT);
  }
}