use crate::soroban_sdk::{self, contracttype, Address};

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
    Wasm,
}

#[derive(Clone)]
#[contracttype]
pub enum UserDataKey {
    OwnedSeriesOrder(Address, u128),
    LastClaim(Address, u128),
}
