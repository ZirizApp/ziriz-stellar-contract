#![no_std]

mod admin_storage;
mod metadata_storage;
mod nft_contract; // for nft module
mod storage_types;
mod test;
mod token_storage;
mod utils;

pub use crate::nft_contract::NonFungibleToken;

smartdeploy_sdk::core_riff!();
