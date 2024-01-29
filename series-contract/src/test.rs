#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::{nft_contract::NonFungibleToken, nft_contract::NonFungibleTokenClient};

fn create_nft_token<'a>(
  env: &Env,
  admin: &Address,
) -> NonFungibleTokenClient<'a> {
  let contract = NonFungibleTokenClient::new(env, &env.register_contract(None, NonFungibleToken {}));
  contract.init(&admin, &String::from_str(env, "Ziris Soroban"), &String::from_str(env, "ZS1"));
  contract
}

#[test]
fn test_mint_sep01() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let contract = create_nft_token(&env, &admin);

    contract.mint(&admin, &user1);

    assert!(contract.balance(&user1) == 1);
    assert!(contract.supply() == 1);
}

#[test]
#[should_panic]
fn test_non_transferable() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let contract = create_nft_token(&env, &admin);

    contract.mint(&admin, &user1);

    assert!(contract.balance(&user1) == 1);
    contract.transfer(&user1, &user2, &1);
}

