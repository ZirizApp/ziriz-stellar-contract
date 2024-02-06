#![no_std]

mod admin_storage;
mod bump;
mod contract; // creagor module
mod data_type;
mod events;
mod metadata_storage;
mod owner_storage;
mod series_storage;
mod share_storage;
mod storage_types;
mod test;
mod token_storage;
mod utils;

pub use crate::contract::ZirizCreator;

smartdeploy_sdk::core_riff!();
