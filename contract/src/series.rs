use soroban_sdk::Env;
use crate::{data_type::Series, metadata::read_metadata, owner::read_creator, storage_types::DataKey};

pub fn calculate_price(env: &Env, id: u128) -> (u128, u128, u128) {
    let fan_base_price = 10;
    let decay_rate : i128= 2;
    let creator_coefficient = 5;
    let price = read_series_price(&env, id);
    let sales = read_series_sales(&env, id);
    let artist_cut = price + (creator_coefficient * sales ) as u128;
    let mut fan_cut = 0;
    if sales > 0 {
        fan_cut = (fan_base_price * (decay_rate.pow(sales as u32)-1))/ decay_rate - 1;
    }
    let total_price = artist_cut + (fan_cut as u128);
    (artist_cut, fan_cut as u128, total_price)
}

pub fn read_series_sales(env: &Env, id: u128) -> u128 {
    let key = DataKey::SeriesSales(id);
    match env.storage().persistent().get::<DataKey, u128>(&key) {
        Some(sales) => sales,
        None => 0,
    }
}

pub fn increment_series_sales(env: &Env, id: u128) {
    let key = DataKey::SeriesSales(id);
    env.storage().persistent().set(&key, &(read_series_sales(&env, id) + 1));
}

pub fn read_series_price(env: &Env, id: u128) -> u128 {
    let key = DataKey::Price(id);
    match env.storage().persistent().get::<DataKey, u128>(&key) {
        Some(price) => price,
        None => 0,
    }
}

pub fn write_series_price(env: &Env, id: u128, price: u128) {
    let key = DataKey::Price(id);
    env.storage().persistent().set(&key, &price);
}

pub fn read_series_info(env: &Env, id: u128) -> Series {
    let series_creator = read_creator(&env, id);
    let metadata = read_metadata(&env, id);
    let (artist_cut, fan_cut, total_price) = calculate_price(&env, id);
    Series {
        creator: series_creator,
        price: total_price,
        metadata,
    }
}