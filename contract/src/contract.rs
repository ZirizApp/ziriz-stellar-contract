use crate::admin::{has_admin, read_admin, write_admin};
use crate::data_type::{Metadata, Series};
use crate::events::{CreateEvent, BuyEvent, ClaimEvent};
use crate::owner::{read_creator, write_creator, write_token_owner, read_token_owner, write_creator_curved};
use crate::metadata::{
    write_name, read_name, 
    write_symbol, read_symbol, 
    read_metadata, write_metadata, map_token_to_series, get_series_id, map_token_to_owner, read_owned_tokens
};
use crate::balance::{read_supply, increment_supply, read_balance, increment_series, read_series, increment_balance, write_native_token, read_native_token, increment_series_balance};
use crate::series::{read_series_info, write_series_price, read_series_sales, increment_series_sales, calculate_price, write_fan_base_price, write_fan_decay_rate, read_fan_base_price, write_sum_fan_cut, write_fan_cut, read_sum_fan_cut};
use crate::share::{map_series_order, read_last_whitdrawn, get_share_balance, write_last_whitdrawn};
use crate::utils::NonFungibleTokenTrait;
use soroban_sdk::token::TokenClient;
use soroban_sdk::{
    contract, contractimpl, Address, Env, String, Vec, log, symbol_short, Symbol, BytesN
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
    
    fn create_series(env: Env, creator: Address, uri: String, base_price: u128, creator_curve: u128, fan_base_price: u128, fan_decay_rate: u128) {
        creator.require_auth();

        assert!(base_price > 0, "Base price must be greater than 0");
        assert!(uri.len() > 0, "URI must be provided");
        assert!(fan_decay_rate <= 1000, "Fan decay rate must be less than 1000");
        
        let next_id = increment_series(&env);
        write_creator(&env, next_id.clone(), &creator);
        let metadata = Metadata{
          short_description_uri: uri.clone(),
          long_description_uri: uri.clone(),
          data_file_uri: uri.clone(),
        };
        write_metadata(&env, next_id.clone(), &metadata);
        write_series_price(&env, next_id.clone(), base_price);
        write_creator_curved(&env, next_id.clone(), creator_curve);
        write_fan_base_price(&env, next_id.clone(), fan_base_price);
        write_fan_decay_rate(&env, next_id.clone(), fan_decay_rate);

        //mint to owner
        let token_id = increment_supply(&env);
        map_token_to_series(&env, token_id, &next_id);
        increment_series_balance(&env, &creator, next_id);
        write_token_owner(&env, token_id, &creator);
        map_token_to_owner(&env, token_id, &creator);

        log!(
            &env,
            "Series {} created by {} with base price {}",
            next_id,
            creator,
            base_price);

        env.events().publish((symbol_short!("create"), next_id), CreateEvent{
          creator,
          series_id: next_id,
          uri,
          base_price,
          creator_curve,
          fan_base_price,
          fan_decay_rate,
        });
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

    fn share_balance(env: Env, account: Address, series_id: u128) -> u128{
        get_share_balance(&env, &account, series_id)
    }

    fn transfer(env: Env, from: Address, to: Address, id: u128) {
      panic!("Can not transfer NFTs");
    }

    fn transfer_from(env: Env, from: Address, to: Address, id: u128) {
      panic!("Can not transfer NFTs");
    }

    fn buy(env: Env, buyer: Address, series_id: u128) {
       buyer.require_auth();

       let fan_base_price = read_fan_base_price(&env, series_id);
       let current_sales = read_series_sales(&env, series_id);
       let prev_fan_cut = read_sum_fan_cut(&env, series_id);
       let (artist_cut, fan_cut, total_price) = calculate_price(&env, series_id, current_sales+1);

       if fan_base_price > 0 {
          assert!(fan_cut>0, "Fun cut must be greater than 0");
          assert!(fan_cut > prev_fan_cut, "Fan cut must be greater than previous fan cut");
       }

       let token_address = read_native_token(&env);
       let token_client = TokenClient::new(&env, &token_address);

       let token_id = increment_supply(&env);

       let creator = read_creator(&env, series_id);
       token_client.transfer(&buyer, &creator, &(artist_cut as i128));
       
       if fan_cut > 0 {
          token_client.transfer(&buyer, &env.current_contract_address(), &(fan_cut as i128));
       }

       increment_balance(&env, &buyer);
       map_token_to_series(&env, token_id, &series_id);
       increment_series_balance(&env, &buyer, series_id);
       write_token_owner(&env, token_id, &buyer);
       map_token_to_owner(&env, token_id, &buyer);
       let series_order = increment_series_sales(&env, series_id);
       map_series_order(&env, &buyer, series_id, series_order);
       write_sum_fan_cut(&env, series_id, fan_cut);
       write_fan_cut(&env, series_id, series_order, fan_cut-prev_fan_cut);

       log!(
           &env,
           "Series {} Token {} bought by {} for {}",
           series_id,
           token_id,
           buyer,
           total_price);
      
      env.events().publish((symbol_short!("buy"), series_id), BuyEvent{
        buyer,
        series_id,
        token_id,
        price: total_price,
        creator_cut: artist_cut,
        fan_cut,
      });
    }

    fn owner(env: Env, token_id: u128) -> Address{
        read_token_owner(&env, token_id)
    }

    fn claim_share(env: Env, account: Address, series_id: u128){
      account.require_auth();

      let last_whitdrawn = read_last_whitdrawn(&env, &account, series_id);
      let current_sales = read_series_sales(&env, series_id);
      assert!(last_whitdrawn < current_sales, "No share to claim");

      let token_address = read_native_token(&env);
      let token_client = TokenClient::new(&env, &token_address);
      let share = get_share_balance(&env, &account, series_id);
      token_client.transfer(&env.current_contract_address(), &account, &(share as i128));
      write_last_whitdrawn(&env, &account, series_id, current_sales);
      log!(
          &env,
          "Share {} claimed by {}",
          share,
          account);
      
      env.events().publish((symbol_short!("claim"), account.clone(), series_id), ClaimEvent{
        owner: account,
        series_id,
        amount: share,
        last_withdrawn: current_sales,
      });
    }

    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
      read_admin(&env).require_auth();
      env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}