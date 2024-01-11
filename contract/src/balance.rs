use soroban_sdk::{Env, Address};
use crate::storage_types::{DataKey, UserDataKey};

pub fn write_native_token(e: &Env, native_token: &Address){
    let key = DataKey::NativeToken;
    e.storage().instance().set(&key, native_token);
}

pub fn read_native_token(e: &Env) -> Address{
    let key = DataKey::NativeToken;
    match e.storage().instance().get::<DataKey, Address>(&key) {
        Some(native_token) => native_token,
        None => panic!("native token not set"),
    }
}

pub fn read_supply(e: &Env) -> u128 {
    let key = DataKey::Series;
    match e.storage().instance().get::<DataKey, u128>(&key) {
        Some(balance) => balance,
        None => 0,
    }
}

pub fn increment_supply(e: &Env) -> u128{
    let key = DataKey::Series;
    let next_supply = read_supply(e) + 1;
    e.storage().instance().set(&key, &next_supply);
    next_supply
}

pub fn read_series(e: &Env) -> u128 {
  let key = DataKey::Series;
  match e.storage().instance().get::<DataKey, u128>(&key) {
      Some(balance) => balance,
      None => 0,
  }
}

pub fn increment_series(e: &Env) -> u128{
  let key = DataKey::Series;
  let next_supply = read_supply(e) + 1;
  e.storage().instance().set(&key, &next_supply);
  next_supply
}

pub fn read_balance(e: &Env, id: &Address) -> u128{
    let key = UserDataKey::Balance(id.clone());
    match e.storage().instance().get::<UserDataKey, u128>(&key) {
        Some(balance) => balance,
        None => 0,
    }
}

pub fn increment_balance(e: &Env, id: &Address){
    let key = UserDataKey::Balance(id.clone());
    e.storage().instance().set(&key, &(read_balance(e, id) + 1));
}

pub fn read_series_balance(e: &Env, account: &Address, id: u128) -> u128{
    let key = UserDataKey::SeriesBalance(account.clone(), id);
    match e.storage().instance().get::<UserDataKey, u128>(&key) {
        Some(balance) => balance,
        None => 0,
    }
}

pub fn increment_series_balance(e: &Env, account: &Address, id: u128){
    let key = UserDataKey::SeriesBalance(account.clone(), id);
    e.storage().instance().set(&key, &(read_series_balance(e, account, id) + 1));
}