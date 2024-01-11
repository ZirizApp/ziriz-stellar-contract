#![no_std]

mod admin;
mod balance;
mod owner;
mod metadata;
mod contract;
mod storage_types;
mod series;
mod share;
mod fans;
mod utils;
mod data_type;
mod test;

pub use crate::contract::NonFungibleTokenClient;

smartdeploy_sdk::core_riff!();
