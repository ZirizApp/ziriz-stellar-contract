use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Name,
    Symbol,
    Series,
    Supply,
}

#[derive(Clone)]
#[contracttype]
pub enum UserDataKey {
    Balance(Address),
}
