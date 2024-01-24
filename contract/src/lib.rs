#![no_std]

mod admin;
mod balance;
mod owner;
mod metadata;
mod contract; // creagor module
mod storage_types;
mod series;
mod share;
mod utils;
mod events;
mod data_type;
// mod nft; // for nft module
mod test;

// pub use crate::nft::NonFungibleToken;
pub use crate::contract::ZirizCreator;

smartdeploy_sdk::core_riff!();
