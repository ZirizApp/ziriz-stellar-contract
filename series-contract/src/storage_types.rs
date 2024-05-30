use crate::soroban_sdk::{self, contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Name,
    Symbol,
    Supply,
}

#[derive(Clone)]
#[contracttype]
pub enum UserDataKey {
    Balance(Address),
}
