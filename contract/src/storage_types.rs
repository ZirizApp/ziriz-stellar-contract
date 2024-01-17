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
    FanBasePrice(u128),
    FanDecayRate(u128),
    SumFanCut(u128),
    FanCut(u128, u128),
    Supply,
}

#[derive(Clone)]
#[contracttype]
pub enum UserDataKey {
    Creator(u128),
    CreatorCurved(u128),
    TokenOwner(u128),
    OwnedTokens(Address),
    OwnedSeriesOrder(Address, u128),
    LastClaim(Address, u128),
    Balance(Address),
    SeriesBalance(Address, u128),
}
