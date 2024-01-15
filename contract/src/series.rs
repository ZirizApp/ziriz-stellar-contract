use core::ops::{Mul, Div};

use soroban_sdk::Env;
use micromath::F32Ext;
use crate::{data_type::Series, metadata::read_metadata, owner::{read_creator, read_creator_curved}, storage_types::DataKey};

pub fn calculate_price(env: &Env, id: u128, sales: u128) -> (u128, u128, u128) {
    let fan_base_price = read_fan_base_price(&env, id);
    let decay_rate = read_fan_decay_rate(&env, id);
    let creator_coefficient = read_creator_curved(&env, id);
    let price = read_series_price(&env, id);
    let artist_cut = price + (creator_coefficient * sales * 10_000_000 ) as u128;
    
    let mut salesmint = sales;
    if salesmint > 50 {
      salesmint = 50;
    }
    let base = 1.0f32;
    let real_decay_rate: f32 = (decay_rate as f32).div(1000.0);
    let one_min_r: f32 = base - real_decay_rate;
    let one_min_rpow_n = base - real_decay_rate.powf(salesmint as f32);
    let fan_base_price_f32: f32 = fan_base_price as f32;
    let alpha_one_min_rpow_n = fan_base_price_f32.div(10_000_000.0f32).mul(one_min_rpow_n);
    let ratio = alpha_one_min_rpow_n.div(one_min_r);
    let total_fan_cut = ratio.mul(10_000_000.0f32) as u128;

    let total_price = artist_cut + total_fan_cut;
    (artist_cut, total_fan_cut, total_price)
}

pub fn get_series_fan_cut(env: &Env, id: u128, order: u128) -> u128 {
    let prev_order = order - 1;
    let fan_base_price = read_fan_base_price(&env, id);
    let decay_rate = read_fan_decay_rate(&env, id);

    let real_decay_rate: f32 = (decay_rate as f32).div(1000.0);
    let decay_rate_pown_min_1 = real_decay_rate.powf(prev_order as f32);
    let wraped_ratio = decay_rate_pown_min_1.mul(10_000_000.0f32);
    let fan_cut = fan_base_price.mul(wraped_ratio as u128).div(10_000_000);
    
    fan_cut
}

pub fn read_series_sales(env: &Env, id: u128) -> u128 {
    let key = DataKey::SeriesSales(id);
    match env.storage().persistent().get::<DataKey, u128>(&key) {
        Some(sales) => sales,
        None => 0,
    }
}

pub fn increment_series_sales(env: &Env, id: u128)->u128{
    let key = DataKey::SeriesSales(id);
    let next_id = read_series_sales(&env, id) + 1;
    env.storage().persistent().set(&key, &next_id);
    next_id
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
    let current_sales = read_series_sales(&env, id);
    let (artist_cut, fan_cut, total_price) = calculate_price(&env, id, current_sales+1);
    let sales = read_series_sales(&env, id);
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
    match env.storage().persistent().get::<DataKey, u128>(&key) {
        Some(price) => price,
        None => 0,
    }
}

pub fn write_fan_base_price(env: &Env, id: u128, price: u128) {
    let key = DataKey::FanBasePrice(id);
    env.storage().persistent().set(&key, &price);
}

pub fn read_fan_decay_rate(env: &Env, id: u128) -> u128 {
    let key = DataKey::FanDecayRate(id);
    match env.storage().persistent().get::<DataKey, u128>(&key) {
        Some(rate) => rate,
        None => 0,
    }
}

pub fn write_fan_decay_rate(env: &Env, id: u128, rate: u128) {
    let key = DataKey::FanDecayRate(id);
    env.storage().persistent().set(&key, &rate);
}