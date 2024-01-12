use soroban_sdk::{
  Address, Env, String, Vec
};

use crate::data_type::{Metadata, Series};

pub trait NonFungibleTokenTrait{
  fn initialize(env: Env, admin: Address, name: String, symbol: String, native_token: Address);
  fn admin(env: Env) -> Address;

  fn name(env: Env) -> String;

  fn symbol(env: Env) -> String;

  fn decimals(env: Env) -> u128;

  fn get_metadata(env: Env, token_id: u128) -> Metadata;

  fn supply(env: Env) -> u128;

  fn number_of_series(env: Env) -> u128;

  fn create_series(env: Env, creator: Address, uri: String, base_price: u128, price_curve: u128);

  fn series_info(env: Env, series_id: u128) -> Series;

  fn owner(env: Env, token_id: u128) -> Address;

  fn series_sales(env: Env, series_id: u128) -> u128;

  fn creator_of(env: Env, series_id: u128) -> Address;

  fn balance(e: Env, account: Address) -> u128;

  fn owned_tokens(e: Env, account: Address) -> Vec<u128>;

  fn share_balance(e: Env, account: Address) -> u128;

  fn transfer(env: Env, from: Address, to: Address, id: u128);

  fn transfer_from(env: Env, from: Address, to: Address, id: u128);

  fn buy(env: Env, buyer: Address, series_id: u128);

  fn claim_share(env: Env, account: Address);
}