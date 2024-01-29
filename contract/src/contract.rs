use crate::admin_storage::{has_admin, read_admin, read_wasm, write_admin, write_wasm};
use crate::data_type::{Metadata, Series};
use crate::events::{BuyEvent, ClaimEvent, CreateEvent};
use crate::metadata_storage::{read_metadata, write_metadata};
use crate::owner_storage::{read_creator, write_creator, write_creator_curved};
use crate::series_storage::{
    calculate_price, increment_series_sales, read_fan_base_price, read_series_info,
    read_series_sales, read_sum_fan_cut, write_fan_base_price, write_fan_cut, write_fan_decay_rate,
    write_series_price, write_sum_fan_cut,
};
use crate::share_storage::{
    get_share_balance, map_series_order, read_last_whitdrawn, write_last_whitdrawn,
};
use crate::token_storage::{
    increment_series, increment_series_balance, read_native_token, read_series, write_native_token,
};
use crate::utils::{Client, ZirizCreatorTrait};
extern crate alloc;
use alloc::format;
use soroban_sdk::token::TokenClient;
use soroban_sdk::{contract, contractimpl, log, symbol_short, Address, BytesN, Env, String};

mod nft_contract {
    soroban_sdk::contractimport!(file = "src/wasm/ziriz-series.wasm");
}

#[contract]
pub struct ZirizCreator;

#[allow(clippy::cast_possible_truncation)]
#[contractimpl]
impl ZirizCreatorTrait for ZirizCreator {
    fn initialize(env: Env, admin: Address, native_token: Address) {
        assert!(!has_admin(&env), "already initialized");

        write_admin(&env, &admin);
        write_native_token(&env, &native_token);
        let wasm_hash = env.deployer().upload_contract_wasm(nft_contract::WASM);
        write_wasm(&env, &wasm_hash);
    }

    fn admin(env: Env) -> Address {
        read_admin(&env)
    }

    fn number_of_series(env: Env) -> u128 {
        read_series(&env)
    }

    #[allow(clippy::cast_possible_wrap)]
    fn create_series(
        env: Env,
        creator: Address,
        uri: String,
        base_price: u128,
        creator_curve: u128,
        fan_base_price: u128,
        fan_decay_rate: u128,
    ) {
        creator.require_auth();

        assert!(base_price > 0, "Base price must be greater than 0");
        assert!(uri.len() > 0, "URI must be provided");
        assert!(
            fan_decay_rate <= 1000,
            "Fan decay rate must be less than 1000"
        );

        let next_id = increment_series(&env);
        write_creator(&env, next_id, &creator);

        let wasm_hash = read_wasm(&env);
        let salt = BytesN::from_array(&env, &[u8::try_from(next_id).expect("Series Overflow"); 32]);

        let deployed_address = env
            .deployer()
            .with_address(creator.clone(), salt)
            .deploy(wasm_hash);

        let symbol = String::from_str(&env, format!("ZS{next_id}").as_str());
        let nft_client = Client::new(&env, &deployed_address);
        nft_client.init(
            &env.current_contract_address(),
            &String::from_str(&env, "Ziris Soroban"),
            &symbol,
        );

        let metadata = Metadata {
            data_file_uri: uri.clone(),
            symbol,
            issuer: deployed_address,
        };
        write_metadata(&env, next_id, &metadata);
        write_series_price(&env, next_id, base_price);
        write_creator_curved(&env, next_id, creator_curve);
        write_fan_base_price(&env, next_id, fan_base_price);
        write_fan_decay_rate(&env, next_id, fan_decay_rate);

        log!(
            &env,
            "Series {} created by {} with base price {}",
            next_id,
            creator,
            base_price
        );

        env.events().publish(
            (symbol_short!("create"), next_id),
            CreateEvent {
                creator,
                series_id: next_id,
                uri,
                base_price,
                creator_curve,
                fan_base_price,
                fan_decay_rate,
            },
        );
    }

    fn series_info(env: Env, series_id: u128) -> Series {
        read_series_info(&env, series_id)
    }

    fn series_sales(env: Env, series_id: u128) -> u128 {
        read_series_sales(&env, series_id)
    }

    fn creator_of(env: Env, series_id: u128) -> Address {
        read_creator(&env, series_id)
    }

    fn share_balance(env: Env, account: Address, series_id: u128) -> u128 {
        get_share_balance(&env, &account, series_id)
    }

    #[allow(clippy::cast_possible_wrap)]
    fn buy(env: Env, buyer: Address, series_id: u128) {
        buyer.require_auth();

        let fan_base_price = read_fan_base_price(&env, series_id);
        let current_sales = read_series_sales(&env, series_id);
        let prev_fan_cut = read_sum_fan_cut(&env, series_id);
        let (artist_cut, fan_cut, total_price) =
            calculate_price(&env, series_id, current_sales + 1);

        if fan_base_price > 0 {
            assert!(fan_cut > 0, "Fun cut must be greater than 0");
            assert!(
                fan_cut > prev_fan_cut,
                "Fan cut must be greater than previous fan cut"
            );
        }

        let token_address = read_native_token(&env);
        let token_client = TokenClient::new(&env, &token_address);

        let creator = read_creator(&env, series_id);
        token_client.transfer(&buyer, &creator, &(artist_cut as i128));

        if fan_cut > 0 {
            token_client.transfer(&buyer, &env.current_contract_address(), &(fan_cut as i128));
        }
        let metadata = read_metadata(&env, series_id);

        let nft_client = Client::new(&env, &metadata.issuer);
        nft_client.mint(&env.current_contract_address(), &buyer);

        increment_series_balance(&env, &buyer, series_id);
        let series_order = increment_series_sales(&env, series_id);
        map_series_order(&env, &buyer, series_id, series_order);
        write_sum_fan_cut(&env, series_id, fan_cut);
        write_fan_cut(&env, series_id, series_order, fan_cut - prev_fan_cut);
        let token_id = series_order;

        log!(
            &env,
            "Series {} Token {} bought by {} for {}",
            series_id,
            token_id,
            buyer,
            total_price
        );

        env.events().publish(
            (symbol_short!("buy"), series_id),
            BuyEvent {
                buyer,
                series_id,
                token_id,
                price: total_price,
                creator_cut: artist_cut,
                fan_cut,
            },
        );
    }

    #[allow(clippy::cast_possible_wrap)]
    fn claim_share(env: Env, account: Address, series_id: u128) {
        account.require_auth();

        let last_whitdrawn = read_last_whitdrawn(&env, &account, series_id);
        let current_sales = read_series_sales(&env, series_id);
        assert!(last_whitdrawn < current_sales, "No share to claim");

        let token_address = read_native_token(&env);
        let token_client = TokenClient::new(&env, &token_address);
        let share = get_share_balance(&env, &account, series_id);
        token_client.transfer(&env.current_contract_address(), &account, &(share as i128));
        write_last_whitdrawn(&env, &account, series_id, current_sales);
        log!(&env, "Share {} claimed by {}", share, account);

        env.events().publish(
            (symbol_short!("claim"), account.clone(), series_id),
            ClaimEvent {
                owner: account,
                series_id,
                amount: share,
                last_withdrawn: current_sales,
            },
        );
    }

    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        read_admin(&env).require_auth();
        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
