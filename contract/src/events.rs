use crate::soroban_sdk::{self, contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug)]
pub struct CreateEvent {
    pub creator: Address,
    pub series_id: u128,
    pub uri: String,
    pub base_price: u128,
    pub creator_curve: u128,
    pub fan_base_price: u128,
    pub fan_decay_rate: u128,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct BuyEvent {
    pub buyer: Address,
    pub series_id: u128,
    pub token_id: u128,
    pub price: u128,
    pub creator_cut: u128,
    pub fan_cut: u128,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct ClaimEvent {
    pub owner: Address,
    pub series_id: u128,
    pub amount: u128,
    pub last_withdrawn: u128,
}
