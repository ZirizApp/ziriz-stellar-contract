use soroban_sdk::Env;

use crate::storage_types::{DataKey, UserDataKey};

pub const DAY_IN_LEDGERS: u32 = 17280;
pub const PERSISTENT_BUMP_CONSTANT: u32 = DAY_IN_LEDGERS * 180;
pub const PERSISTENT_BUMP_CONSTANT_THRESHOLD: u32 = DAY_IN_LEDGERS * 90;

pub fn extend_instance(env: &Env) {
   extend_persistent(env, &DataKey::Admin);
   extend_persistent(env, &DataKey::Supply);
   extend_persistent(env, &DataKey::Symbol);
   extend_persistent(env, &DataKey::Name);
}

pub fn extend_persistent(env: &Env, key: &DataKey) {
  if env.storage().persistent().has(key) {
      env.storage().persistent().extend_ttl(
          key,
          PERSISTENT_BUMP_CONSTANT_THRESHOLD,
          PERSISTENT_BUMP_CONSTANT,
      );
  }
}

pub fn extend_user_persistent(env: &Env, key: &UserDataKey) {
  if env.storage().persistent().has(key) {
      env.storage().persistent().extend_ttl(key, PERSISTENT_BUMP_CONSTANT_THRESHOLD, PERSISTENT_BUMP_CONSTANT);
  }
}