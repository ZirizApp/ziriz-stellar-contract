use crate::admin::{has_admin, read_admin, write_admin};
use crate::data_type::{Metadata, Series};
use crate::fans::{is_already_fan, add_fans};
use crate::owner::{read_creator, write_creator, write_token_owner, read_token_owner};
use crate::metadata::{
    write_name, read_name, 
    write_symbol, read_symbol, 
    read_metadata, write_metadata, map_token_to_series, get_series_id, map_token_to_owner, read_owned_tokens
};
use crate::balance::{read_supply, increment_supply, read_balance, increment_series, read_series, increment_balance, write_native_token, read_native_token, increment_series_balance};
use crate::series::{read_series_info, write_series_price, read_series_sales, increment_series_sales, calculate_price};
use crate::share::{read_share, distribute_share, remove_share};
use crate::utils::NonFungibleTokenTrait;
use soroban_sdk::token::TokenClient;
use soroban_sdk::{
    contract, contractimpl, Address, Env, String, Vec
};

#[contract]
pub struct NonFungibleToken;

#[contractimpl]
impl NonFungibleTokenTrait for NonFungibleToken {
    fn initialize(env: Env, admin: Address, name: String, symbol: String, native_token: Address) {
        if has_admin(&env) {
            panic!("already initialized")
        }
        
        write_admin(&env, &admin);
        write_name(&env, &name);
        write_symbol(&env, &symbol);
        write_native_token(&env, &native_token);
    }

    fn admin(env: Env)->Address{
        read_admin(&env)
    }

    fn name(env: Env) -> String {
        read_name(&env)
    }

    fn symbol(env: Env) -> String {
        read_symbol(&env)
    }

    fn decimals(env: Env) -> u128 {
        0
    }

    fn get_metadata(env: Env, token_id: u128) -> Metadata {
        let series = get_series_id(&env, token_id);
        read_metadata(&env, series)
    }

    fn supply(env: Env) -> u128 {
        read_supply(&env)
    }

    fn number_of_series(env: Env) -> u128{
        read_series(&env)
    }
    
    fn create_series(env: Env, creator: Address, uri: String, base_price: u128) {
        creator.require_auth();

        assert!(base_price > 0, "Base price must be greater than 0");
        assert!(uri.len() > 0, "URI must be provided");
        
        let next_id = increment_series(&env);
        write_creator(&env, next_id.clone(), &creator);
        write_metadata(&env, next_id.clone(), &Metadata{
            short_description_uri: uri.clone(),
            long_description_uri: uri.clone(),
            data_file_uri: uri.clone(),
        });
        write_series_price(&env, next_id.clone(), base_price);
    }

    fn series_info(env: Env, series_id: u128) -> Series{
        read_series_info(&env, series_id)
    }

    fn series_sales(env: Env, series_id: u128) -> u128{
        read_series_sales(&env, series_id)
    }

    fn creator_of(env: Env, series_id: u128) -> Address{
        read_creator(&env, series_id)
    }

    fn balance(env: Env, account: Address) -> u128{
        read_balance(&env, &account)
    }
    
    fn owned_tokens(e: Env, account: Address) -> Vec<u128>{
        read_owned_tokens(&e, &account)
    }

    fn share_balance(env: Env, account: Address) -> u128{
      read_share(&env, &account)
    }

    fn transfer(env: Env, from: Address, to: Address, id: u128) {
      panic!("Can not transfer NFTs");
    }

    fn transfer_from(env: Env, from: Address, to: Address, id: u128) {
      panic!("Can not transfer NFTs");
    }

    fn buy(env: Env, buyer: Address, series_id: u128) {
       buyer.require_auth();

       let  (artist_cut, fan_cut, total_price) = calculate_price(&env, series_id);
       let sales = read_series_sales(&env, series_id);

       if sales > 0 && fan_cut == 0 {
           panic!("Secondary sales Fans Cut is 0");
       }

       let token_address = read_native_token(&env);
       let token_client = TokenClient::new(&env, &token_address);

       let token_id = increment_supply(&env);

       let creator = read_creator(&env, series_id);
       token_client.transfer(&buyer, &creator, &(artist_cut as i128));
       
       if fan_cut > 0 {
          token_client.transfer(&buyer, &env.current_contract_address(), &(fan_cut as i128));
          distribute_share(&env, series_id, &fan_cut);
       }

       map_token_to_series(&env, token_id, &series_id);
       increment_balance(&env, &buyer);
       increment_series_balance(&env, &buyer);
       write_token_owner(&env, token_id, &buyer);
       map_token_to_owner(&env, token_id, &buyer);
       increment_series_sales(&env, series_id);
       add_fans(&env, series_id, &buyer);
    }

    fn owner(env: Env, token_id: u128) -> Address{
        read_token_owner(&env, token_id)
    }

    fn claim_share(env: Env, account: Address){
      account.require_auth();

      assert!(read_share(&env, &account) > 0, "You have no share to claim");

      let token_address = read_native_token(&env);
      let token_client = TokenClient::new(&env, &token_address);
      let share = read_share(&env, &account);
      token_client.transfer(&env.current_contract_address(), &account, &(share as i128));
      remove_share(&env, &account, share);
    }
}