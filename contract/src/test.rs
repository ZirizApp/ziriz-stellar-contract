#![cfg(test)]

extern crate std;

use crate::{contract::NonFungibleToken, NonFungibleTokenClient};
use soroban_sdk::{
    testutils::{Address as _, Logs},
    Address, Env, IntoVal, String, token::{TokenClient, StellarAssetClient, self},
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

    nft.create_series(&user1, &String::from_str(&env,"https://www.ziriz.com/1"), &1000);
    nft.create_series(&user2,&String::from_str(&env,"https://www.ziriz.com/2"), &1000);
    nft.create_series(&user1, &String::from_str(&env,"https://www.ziriz.com/3"), &1000);

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

    nft.create_series(&user1, &String::from_str(&env,"https://www.ziriz.com/1"), &1000);

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
    let (token, token_admin) = create_token(&env, &admin);
    let nft = create_ziriz_app(&env, &admin, &token.address);

    nft.create_series(&user1, &String::from_str(&env,"https://www.ziriz.com/1"), &1000);
    assert_eq!(nft.creator_of(&1), user1);

    token_admin.mint(&user2, &2000);
    assert_eq!(token.balance(&user2), 2000);
    assert_eq!(nft.series_info(&1).price, 1000);
    nft.buy(&user2, &1);
    debug_assert_eq!(token.balance(&user2) , 1000);
    assert_eq!(nft.balance(&user2), 1);
    assert_eq!(nft.series_sales(&1), 1);

    assert!(nft.series_info(&1).price > 1000);
    token_admin.mint(&user3, &4000);
    assert_eq!(token.balance(&user3), 4000);
    nft.buy(&user3, &1);
    assert_eq!(nft.series_sales(&1), 2);

    assert!(nft.share_balance(&user2)>0);

    nft.claim_share(&user2);

    assert_eq!(nft.share_balance(&user2), 0);

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

    nft.create_series(&user1, &String::from_str(&env,"https://www.ziriz.com/1"), &1000);

    assert_eq!(nft.creator_of(&1), user1);

    nft.transfer(&user1, &user2, &1);

    std::println!("{}", env.logs().all().join("\n"));
}
