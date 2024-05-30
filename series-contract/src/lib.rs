#![no_std]

pub(crate) use loam_sdk::soroban_sdk;

mod admin_storage;
mod metadata_storage;
mod bump;
mod nft_contract; // for nft module
mod storage_types;
mod test;
mod token_storage;
mod utils;

pub use crate::nft_contract::NonFungibleToken;

smartdeploy_sdk::core_riff!();
