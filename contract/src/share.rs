use soroban_sdk::{Env, Address};
use crate::{storage_types::{DataKey, UserDataKey}, fans::read_fans, balance::read_series_balance};

pub fn read_share(e: &Env, address: &Address) -> u128{
  let key = UserDataKey::Share(address.clone());
  match e.storage().instance().get::<UserDataKey, u128>(&key) {
      Some(share) => share,
      None => 0,
  }
}

pub fn add_share(e: &Env, address: &Address, amount: u128){
  let key = UserDataKey::Share(address.clone());
  e.storage().instance().set(&key, &(read_share(e, address) + amount));
}

pub fn remove_share(e: &Env, address: &Address, amount: u128){
  let key = UserDataKey::Share(address.clone());
  e.storage().instance().set(&key, &(read_share(e, address) - amount));
}

pub fn distribute_share(e: &Env, id: u128, amount: &u128){
  let fans = read_fans(&e, id);
  let total_share = fans.len() as u128;
  for fan in fans.iter() {
    let balance = read_series_balance(e, &fan, id);
    add_share(&e, &fan, ( (amount / total_share) * balance)  as u128);
  }
}