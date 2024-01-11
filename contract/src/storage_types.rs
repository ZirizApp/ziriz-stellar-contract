use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Name,
    Symbol,
    Metadata(u128),
    Owner(u128),
    Token(u128),
    Price(u128),
    NativeToken,
    Series,
    SeriesSales(u128),
    Fans(u128),
    Supply,
}

#[derive(Clone)]
#[contracttype]
pub enum UserDataKey {
    Creator(u128),
    TokenOwner(u128),
    OwnedTokens(Address),
    Balance(Address),
    SeriesBalance(Address),
    Share(Address),
}
