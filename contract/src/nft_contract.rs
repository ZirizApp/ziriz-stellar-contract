use crate::admin_storage::{has_admin, read_admin, write_admin};
use crate::metadata_storage::{read_name, read_symbol, write_name, write_symbol};
use crate::token_data::{increment_balance, increment_supply, read_balance, read_supply};
use crate::utils::NonFungibleTokenTrait;
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct NonFungibleToken;

#[contractimpl]
impl NonFungibleTokenTrait for NonFungibleToken {
    fn init(env: Env, admin: Address, name: String, symbol: String) {
        assert!(!has_admin(&env), "already initialized");

        write_admin(&env, &admin);
        write_name(&env, &name);
        write_symbol(&env, &symbol);
    }

    fn admin(env: Env) -> Address {
        read_admin(&env)
    }

    fn name(env: Env) -> String {
        read_name(&env)
    }

    fn symbol(env: Env) -> String {
        read_symbol(&env)
    }

    fn decimals(_env: Env) -> u128 {
        0
    }

    fn mint(env: Env, admin: Address, to: Address) {
        assert!(admin == read_admin(&env), "Only admin can mint");

        admin.require_auth();

        increment_balance(&env, &to);
        increment_supply(&env);
    }

    fn supply(env: Env) -> u128 {
        read_supply(&env)
    }

    #[allow(clippy::cast_possible_wrap)]
    fn balance(env: Env, account: Address) -> i128 {
        read_balance(&env, &account) as i128
    }

    fn transfer(_env: Env, from: Address, to: Address, id: u128) {
        panic!("Can not transfer NFTs");
    }

    fn transfer_from(_env: Env, from: Address, to: Address, id: u128) {
        panic!("Can not transfer NFTs");
    }
}
