#![no_std]

mod admin;
mod balance;
mod contract;
mod data_type;
mod events;
mod metadata;
mod owner;
mod series;
mod share;
mod storage_types;
mod test;
mod utils;

pub use crate::contract::NonFungibleTokenClient;

smartdeploy_sdk::core_riff!();
