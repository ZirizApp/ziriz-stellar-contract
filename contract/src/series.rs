use core::ops::{Mul, Div, Add};

use soroban_sdk::{Env, U256};
use crate::{data_type::Series, metadata::read_metadata, owner::{read_creator, read_creator_curved}, storage_types::DataKey};

pub fn calculate_price(env: &Env, id: u128, sales: u128) -> (u128, u128, u128) {
    let fan_base_price = read_fan_base_price(&env, id);
    let decay_rate = read_fan_decay_rate(&env, id);
    let creator_coefficient = read_creator_curved(&env, id);
    let price = read_series_price(&env, id);
    let artist_cut = price + (creator_coefficient * sales * 10_000_000 ) as u128;
    let mut total_fan_cut : u128= read_sum_fan_cut(env, id); // read last fan cut
    let prev_fan_cut = read_fan_cut(env, id, sales-1);
    if total_fan_cut == 0 {
      total_fan_cut = fan_base_price;
    }else{
      total_fan_cut = total_fan_cut.add(prev_fan_cut.mul(decay_rate).div(1000));
    }

    let total_price = artist_cut + total_fan_cut;
    (artist_cut, total_fan_cut , total_price)
}

pub fn get_limited_sum(env: &Env, decay_rate: u128, fan_base_price: u128, salesmint: u128) -> u128 {
    let base=U256::from_u32(&env, 1000);
    let decay_rate_u256 = U256::from_u128(&env, decay_rate);
    let one_min_r = base.sub(&decay_rate_u256);
    let one_min_rpow_n = base.pow(salesmint as u32).sub(&decay_rate_u256.pow(salesmint as u32));
    let alpha_one_min_rpow_n = U256::from_u128(&env, fan_base_price).mul(&one_min_rpow_n);
    let ratio = alpha_one_min_rpow_n.div(&one_min_r);
    let total_fan_cut_u256 = ratio.div(&base.pow(salesmint as u32)).mul(&base);
    let total_fan_cut = total_fan_cut_u256.to_u128().unwrap();
    total_fan_cut
}

pub fn get_unlimited_sum(env: &Env, decay_rate: u128, fan_base_price: u128) -> u128 {
    let base: u128 = 1000;
    let one_min_r = base - decay_rate;
    let total_fan_cut = fan_base_price.div(one_min_r).mul(base);
    total_fan_cut
}

pub fn get_series_fan_cut(env: &Env, id: u128, order: u128) -> u128 {
    let prev_order = order - 1;
    let fan_base_price = read_fan_base_price(&env, id);
    let decay_rate = read_fan_decay_rate(&env, id);

    if order <= 20 {
      let base= U256::from_u128(&env, 1000);
      let decay_rate_u256 = U256::from_u128(&env, decay_rate);
      let decay_rate_pown_min_1 = decay_rate_u256.pow(prev_order as u32);
      let fan_cut = U256::from_u128(&env, fan_base_price).mul(&decay_rate_pown_min_1).div(&base.pow(prev_order as u32));
      fan_cut.to_u128().unwrap()
    }else{
      let capped_fan_cut = get_unlimited_sum(env, decay_rate, fan_base_price); 
      let early_20= get_limited_sum(env, decay_rate, fan_base_price, 20);
      let balance_left = capped_fan_cut - early_20;
      balance_left.div(10_000)
    }
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

pub fn read_sum_fan_cut(env: &Env, id: u128) -> u128 {
    let key = DataKey::SumFanCut(id);
    match env.storage().persistent().get::<DataKey, u128>(&key) {
        Some(price) => price,
        None => 0,
    }
}

pub fn write_sum_fan_cut(env: &Env, id: u128, price: u128) {
    let key = DataKey::SumFanCut(id);
    env.storage().persistent().set(&key, &price);
}

pub fn read_fan_cut(env: &Env, id: u128, order: u128) -> u128 {
    let key = DataKey::FanCut(id, order);
    match env.storage().persistent().get::<DataKey, u128>(&key) {
        Some(price) => price,
        None => 0,
    }
}

pub fn write_fan_cut(env: &Env, id: u128, order: u128, price: u128) {
    let key = DataKey::FanCut(id, order);
    env.storage().persistent().set(&key, &price);
}