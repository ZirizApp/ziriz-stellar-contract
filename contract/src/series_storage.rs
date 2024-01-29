use core::ops::{Add, Div, Mul};

use crate::{
    data_type::Series,
    metadata_storage::read_metadata,
    owner_storage::{read_creator, read_creator_curved},
    storage_types::DataKey,
};
use soroban_sdk::Env;

pub fn calculate_price(env: &Env, id: u128, sales: u128) -> (u128, u128, u128) {
    let fan_base_price = read_fan_base_price(env, id);
    let decay_rate = read_fan_decay_rate(env, id);
    let creator_coefficient = read_creator_curved(env, id);
    let price = read_series_price(env, id);
    let artist_cut = price + (creator_coefficient * sales);
    let mut total_fan_cut: u128 = read_sum_fan_cut(env, id); // read last fan cut
    let prev_fan_cut = read_fan_cut(env, id, sales - 1);
    if total_fan_cut == 0 {
        total_fan_cut = fan_base_price;
    } else {
        total_fan_cut = total_fan_cut.add(prev_fan_cut.mul(decay_rate).div(1000));
    }

    let total_price = artist_cut + total_fan_cut;
    (artist_cut, total_fan_cut, total_price)
}

pub fn read_series_sales(env: &Env, id: u128) -> u128 {
    let key = DataKey::SeriesSales(id);
    env.storage()
        .persistent()
        .get::<DataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn increment_series_sales(env: &Env, id: u128) -> u128 {
    let key = DataKey::SeriesSales(id);
    let next_id = read_series_sales(env, id) + 1;
    env.storage().persistent().set(&key, &next_id);
    next_id
}

pub fn read_series_price(env: &Env, id: u128) -> u128 {
    let key = DataKey::Price(id);
    env.storage()
        .persistent()
        .get::<DataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn write_series_price(env: &Env, id: u128, price: u128) {
    let key = DataKey::Price(id);
    env.storage().persistent().set(&key, &price);
}

pub fn read_series_info(env: &Env, id: u128) -> Series {
    let series_creator = read_creator(env, id);
    let metadata = read_metadata(env, id);
    let current_sales = read_series_sales(env, id);
    let (artist_cut, fan_cut, total_price) = calculate_price(env, id, current_sales + 1);
    let sales = read_series_sales(env, id);
    Series {
        creator: series_creator,
        price: total_price,
        artist_cut,
        fan_cut,
        metadata,
        sales,
    }
}

pub fn read_fan_base_price(env: &Env, id: u128) -> u128 {
    let key = DataKey::FanBasePrice(id);
    env.storage()
        .persistent()
        .get::<DataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn write_fan_base_price(env: &Env, id: u128, price: u128) {
    let key = DataKey::FanBasePrice(id);
    env.storage().persistent().set(&key, &price);
}

pub fn read_fan_decay_rate(env: &Env, id: u128) -> u128 {
    let key = DataKey::FanDecayRate(id);
    env.storage()
        .persistent()
        .get::<DataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn write_fan_decay_rate(env: &Env, id: u128, rate: u128) {
    let key = DataKey::FanDecayRate(id);
    env.storage().persistent().set(&key, &rate);
}

pub fn read_sum_fan_cut(env: &Env, id: u128) -> u128 {
    let key = DataKey::SumFanCut(id);
    env.storage()
        .persistent()
        .get::<DataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn write_sum_fan_cut(env: &Env, id: u128, price: u128) {
    let key = DataKey::SumFanCut(id);
    env.storage().persistent().set(&key, &price);
}

pub fn read_fan_cut(env: &Env, id: u128, order: u128) -> u128 {
    let key = DataKey::FanCut(id, order);
    env.storage()
        .persistent()
        .get::<DataKey, u128>(&key)
        .unwrap_or(0)
}

pub fn write_fan_cut(env: &Env, id: u128, order: u128, price: u128) {
    let key = DataKey::FanCut(id, order);
    env.storage().persistent().set(&key, &price);
}
