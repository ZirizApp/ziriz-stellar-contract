use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Metadata(u128),
    Price(u128),
    NativeToken,
    Series,
    SeriesSales(u128),
    FanBasePrice(u128),
    FanDecayRate(u128),
    SumFanCut(u128),
    FanCut(u128, u128),
    Creator(u128),
    CreatorCurved(u128),
    TokenSeries(u128),
    Token
}

#[derive(Clone)]
#[contracttype]
pub enum UserDataKey {
    OwnedSeriesOrder(Address, u128),
    LastClaim(Address, u128),
}
