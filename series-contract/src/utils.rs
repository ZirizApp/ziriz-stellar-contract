use soroban_sdk::{Address, Env, String};

pub trait NonFungibleTokenTrait {
    fn init(env: Env, admin: Address, name: String, symbol: String);
    fn admin(env: Env) -> Address;

    fn name(env: Env) -> String;

    fn symbol(env: Env) -> String;

    fn decimals(env: Env) -> u128;

    fn supply(env: Env) -> u128;

    fn balance(env: Env, account: Address) -> i128;

    fn transfer(env: Env, from: Address, to: Address, id: u128);

    fn transfer_from(env: Env, from: Address, to: Address, id: u128);

    fn mint(env: Env, admin: Address, to: Address);
}
