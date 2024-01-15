use core::ops::{Div, Add, Mul};

use soroban_sdk::{Env, Address, Vec};
use crate::{storage_types::UserDataKey, series::{read_series_sales, get_series_fan_cut}};

pub fn read_series_order(env: &Env, account: &Address, id: u128) -> Vec<u128> {
    let key = UserDataKey::OwnedSeriesOrder(account.clone(), id);
    match env.storage().persistent().get(&key) {
        Some(tokens) => tokens,
        None => Vec::new(&env),
    }
}

pub fn map_series_order(env: &Env, account: &Address, id: u128, order_id: u128){
    let key = UserDataKey::OwnedSeriesOrder(account.clone(), id);
    let mut tokens = match env.storage().persistent().get(&key) {
        Some(tokens) => tokens,
        None => Vec::new(&env),
    };
    tokens.push_back(order_id);
    env.storage().persistent().set(&key, &tokens);
}

pub fn read_last_whitdrawn(env: &Env, account: &Address, id: u128) -> u128 {
    let key = UserDataKey::LastWridrawn(account.clone(), id);
    match env.storage().persistent().get(&key) {
        Some(last_whitdrawn) => last_whitdrawn,
        None => 0,
    }
}

pub fn write_last_whitdrawn(env: &Env, account: &Address, id: u128, last_whitdrawn: u128) {
    let key = UserDataKey::LastWridrawn(account.clone(), id);
    env.storage().persistent().set(&key, &last_whitdrawn);
}

pub fn get_share_balance(env: &Env, account: &Address, id: u128) -> u128 {
  let mut share: u128 = 0;
  let orders = read_series_order(&env, &account, id);
  let current_sales = read_series_sales(&env, id);
  let last_whitdrawn = read_last_whitdrawn(&env, &account, id);
  for order in orders.iter() {
    if last_whitdrawn < order+1{
      let fan_cut = get_series_fan_cut(&env, id, order+1); // next fan cut after this order 
      share += fan_cut.mul(current_sales - order); // your fan cut * number of sales since last whitdrawn
    }
  }
  share
}
