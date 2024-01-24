
use soroban_sdk::{
  contractclient, Address, BytesN, Env, String
};

use crate::data_type::{Series};

#[doc(hidden)]
#[deprecated(note = "use NFTClient")]
pub use NFTClient as Client;

#[contractclient(name = "NFTClient")]
pub trait NonFungibleTokenTrait{
  fn init(env: Env, admin: Address, name: String, symbol: String);
  fn admin(env: Env) -> Address;

  fn name(env: Env) -> String;

  fn symbol(env: Env) -> String;

  fn decimals(env: Env) -> u128;

  fn supply(env: Env) -> u128;

  fn balance(e: Env, account: Address) -> i128;

  fn transfer(env: Env, from: Address, to: Address, id: u128);

  fn transfer_from(env: Env, from: Address, to: Address, id: u128);

  fn mint(env: Env, admin: Address, to: Address);
}

pub trait ZirizCreatorTrait{
  fn initialize(env: Env, admin: Address, native_token: Address);
  fn admin(env: Env) -> Address;

  fn number_of_series(env: Env) -> u128;

  fn create_series(env: Env, creator: Address, uri: String, base_price: u128, name: String, description: String, creator_curve: u128, fan_base_price: u128, fan_decay_rate: u128);

  fn series_info(env: Env, series_id: u128) -> Series;

  fn series_sales(env: Env, series_id: u128) -> u128;

  fn creator_of(env: Env, series_id: u128) -> Address;

  fn share_balance(e: Env, account: Address, series_id: u128) -> u128;

  fn buy(env: Env, buyer: Address, series_id: u128);

  fn claim_share(env: Env, account: Address, series_id: u128);

  fn upgrade(env: Env, new_wasm_hash: BytesN<32>);
}

