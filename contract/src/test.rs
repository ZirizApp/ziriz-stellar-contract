#![cfg(test)]

extern crate std;

use core::ops::Add;

use crate::{contract::NonFungibleToken, NonFungibleTokenClient};
use soroban_sdk::{
    testutils::{Address as _, Logs},
    Address, Env, IntoVal, String, token::{TokenClient, StellarAssetClient},
};

fn create_ziriz_app<'a>(env: &Env, admin: &Address, native_token: &Address) -> NonFungibleTokenClient<'a> {
    let contract = NonFungibleTokenClient::new(env, &env.register_contract(None, NonFungibleToken {}));
    contract.initialize(admin, &"Ziriz NFT".into_val(env), &"Ziriz".into_val(env), &native_token);
    contract
}

fn create_token<'a>(env: &Env, admin: &Address) -> (TokenClient<'a>, StellarAssetClient<'a>){
  let contract_address = env.register_stellar_asset_contract(admin.clone());
  (
    TokenClient::new(env, &contract_address),
    StellarAssetClient::new(env, &contract_address),
  )
}


#[test]
fn test_create_series() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);
    let nft = create_ziriz_app(&env, &admin, &token.address);

    nft.create_series(&user1, &String::from_str(&env,"https://www.ziriz.com/1"), &10_000_000,&1,&100_000_000, &900);
    nft.create_series(&user2,&String::from_str(&env,"https://www.ziriz.com/2"), &10_000_000,&1,&100_000_000, &900);
    nft.create_series(&user1, &String::from_str(&env,"https://www.ziriz.com/3"), &10_000_000,&1, &100_000_000, &900);

    assert_eq!(nft.number_of_series(), 3);
    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_creator() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);
    let nft = create_ziriz_app(&env, &admin, &token.address);

    nft.create_series(&user1, &String::from_str(&env,"https://www.ziriz.com/1"), &10_000_000,&1, &100_000_000, &900);

    assert_eq!(nft.creator_of(&1), user1);
    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_buy_series_and_claim() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let init_balance = 10_000_000;
    let (token, token_admin) = create_token(&env, &admin);
    let nft = create_ziriz_app(&env, &admin, &token.address);

    nft.create_series(&user1, &String::from_str(&env,"https://www.ziriz.com/1"), &10_000_000,&1, &100_000_000, &900);
    assert_eq!(nft.creator_of(&1), user1);

    let first_series_info = nft.series_info(&1);
    let nft_1_price = first_series_info.price as i128;
    token_admin.mint(&user2, &(nft_1_price+init_balance));
    assert_eq!(token.balance(&user2), (nft_1_price+init_balance));
    nft.buy(&user2, &1);
    assert_eq!(nft.balance(&user2), 1);
    assert_eq!(nft.series_sales(&1), 1);
    assert_eq!(nft.owner(&2), user2);
    let user2tokens = nft.owned_tokens(&user2);
    assert_eq!(user2tokens.len(), 1);
    assert_eq!(user2tokens.get(0), Some(2));
    let metadata = nft.get_metadata(&1);
    assert_eq!(metadata.data_file_uri, String::from_str(&env,"https://www.ziriz.com/1"));

    let second_series_info = nft.series_info(&1);
    let nft_2_price = second_series_info.price as i128;
    assert!(second_series_info.fan_cut > first_series_info.fan_cut);
    assert!(nft.series_info(&1).price > 10_000_000);
    token_admin.mint(&user3, &(nft_2_price+init_balance));
    assert_eq!(token.balance(&user3), (nft_2_price+init_balance));
    nft.buy(&user3, &1);
    assert_eq!(nft.series_sales(&1), 2);
    assert_eq!(nft.owner(&3), user3);
    let user3tokens = nft.owned_tokens(&user3);
    assert_eq!(user3tokens.len(), 1);
    assert_eq!(user3tokens.get(0), Some(3));
    let metadata2 = nft.get_metadata(&3);
    assert_eq!(metadata2.data_file_uri, String::from_str(&env,"https://www.ziriz.com/1"));

    let mut last_price = nft.series_info(&1).price;

    for i in 0..10{
        let anon_user = Address::generate(&env);
        let to_top_up = last_price.add(init_balance as u128);
        token_admin.mint(&anon_user, &(to_top_up as i128));
        nft.buy(&anon_user, &1);
        let new_price = nft.series_info(&1).price;
        assert!(new_price > last_price);
        assert!(token.balance(&anon_user) < to_top_up as i128);
        last_price = new_price;
    }

    assert!(nft.share_balance(&user2, &1) > nft.share_balance(&user3, &1));
    assert_eq!(nft.share_balance(&user2, &1), 90_000_000 * 11);
    nft.claim_share(&user2, &1);
    assert_eq!(nft.share_balance(&user2, &1), 0);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
#[should_panic(expected = "Can not transfer NFTs")]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);
    let nft = create_ziriz_app(&env, &admin, &token.address);

    nft.create_series(&user1, &String::from_str(&env,"https://www.ziriz.com/1"), &10_000_000,&1, &100_000_000, &900);

    assert_eq!(nft.creator_of(&1), user1);

    nft.transfer(&user1, &user2, &1);

    std::println!("{}", env.logs().all().join("\n"));
}
