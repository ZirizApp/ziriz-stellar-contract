import { ContractSpec, Address } from 'stellar-sdk';
import { Buffer } from "buffer";
import { AssembledTransaction, Ok, Err } from './assembled-tx.js';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
  Error_,
  Result,
} from './assembled-tx.js';
import type { ClassOptions, XDR_BASE64 } from './method-options.js';

export * from './assembled-tx.js';
export * from './method-options.js';

if (typeof window !== 'undefined') {
    //@ts-ignore Buffer exists
    window.Buffer = window.Buffer || Buffer;
}


export const networks = {
    testnet: {
        networkPassphrase: "Test SDF Network ; September 2015",
        contractId: "CBP55WUCDFIDNXRXMYYA6ENSBJP2CUBDQ5HRKY4ADBUWK63ASMT5GVNA",
    }
} as const

/**
    
    */
export type DataKey = {tag: "Admin", values: void} | {tag: "Name", values: void} | {tag: "Symbol", values: void} | {tag: "Metadata", values: readonly [u128]} | {tag: "Owner", values: readonly [u128]} | {tag: "Token", values: readonly [u128]} | {tag: "Price", values: readonly [u128]} | {tag: "NativeToken", values: void} | {tag: "Series", values: void} | {tag: "SeriesSales", values: readonly [u128]} | {tag: "FanBasePrice", values: readonly [u128]} | {tag: "FanDecayRate", values: readonly [u128]} | {tag: "SumFanCut", values: readonly [u128]} | {tag: "FanCut", values: readonly [u128, u128]} | {tag: "Supply", values: void} | {tag: "Wasm", values: void};

/**
    
    */
export type UserDataKey = {tag: "Creator", values: readonly [u128]} | {tag: "CreatorCurved", values: readonly [u128]} | {tag: "TokenOwner", values: readonly [u128]} | {tag: "OwnedTokens", values: readonly [string]} | {tag: "OwnedSeriesOrder", values: readonly [string, u128]} | {tag: "LastClaim", values: readonly [string, u128]} | {tag: "Balance", values: readonly [string]} | {tag: "SeriesBalance", values: readonly [string, u128]};

/**
    
    */
export interface CreateEvent {
  /**
    
    */
base_price: u128;
  /**
    
    */
creator: string;
  /**
    
    */
creator_curve: u128;
  /**
    
    */
fan_base_price: u128;
  /**
    
    */
fan_decay_rate: u128;
  /**
    
    */
series_id: u128;
  /**
    
    */
uri: string;
}

/**
    
    */
export interface BuyEvent {
  /**
    
    */
buyer: string;
  /**
    
    */
creator_cut: u128;
  /**
    
    */
fan_cut: u128;
  /**
    
    */
price: u128;
  /**
    
    */
series_id: u128;
  /**
    
    */
token_id: u128;
}

/**
    
    */
export interface ClaimEvent {
  /**
    
    */
amount: u128;
  /**
    
    */
last_withdrawn: u128;
  /**
    
    */
owner: string;
  /**
    
    */
series_id: u128;
}

/**
    
    */
export interface Metadata {
  /**
    
    */
data_file_uri: string;
  /**
    
    */
issuer: string;
  /**
    
    */
symbol: string;
}

/**
    
    */
export interface Series {
  /**
    
    */
artist_cut: u128;
  /**
    
    */
creator: string;
  /**
    
    */
fan_cut: u128;
  /**
    
    */
metadata: Metadata;
  /**
    
    */
price: u128;
  /**
    
    */
sales: u128;
}

/**
    
    */
export const Errors = {

}

export class Contract {
    spec: ContractSpec;
    constructor(public readonly options: ClassOptions) {
        this.spec = new ContractSpec([
            "AAAAAAAAAAAAAAAKaW5pdGlhbGl6ZQAAAAAAAgAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAAAxuYXRpdmVfdG9rZW4AAAATAAAAAA==",
        "AAAAAAAAAAAAAAAFYWRtaW4AAAAAAAAAAAAAAQAAABM=",
        "AAAAAAAAAAAAAAAQbnVtYmVyX29mX3NlcmllcwAAAAAAAAABAAAACg==",
        "AAAAAAAAAAAAAAANY3JlYXRlX3NlcmllcwAAAAAAAAYAAAAAAAAAB2NyZWF0b3IAAAAAEwAAAAAAAAADdXJpAAAAABAAAAAAAAAACmJhc2VfcHJpY2UAAAAAAAoAAAAAAAAADWNyZWF0b3JfY3VydmUAAAAAAAAKAAAAAAAAAA5mYW5fYmFzZV9wcmljZQAAAAAACgAAAAAAAAAOZmFuX2RlY2F5X3JhdGUAAAAAAAoAAAAA",
        "AAAAAAAAAAAAAAALc2VyaWVzX2luZm8AAAAAAQAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAEAAAfQAAAABlNlcmllcwAA",
        "AAAAAAAAAAAAAAAMc2VyaWVzX3NhbGVzAAAAAQAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAEAAAAK",
        "AAAAAAAAAAAAAAAKY3JlYXRvcl9vZgAAAAAAAQAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAEAAAAT",
        "AAAAAAAAAAAAAAANc2hhcmVfYmFsYW5jZQAAAAAAAAIAAAAAAAAAB2FjY291bnQAAAAAEwAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAEAAAAK",
        "AAAAAAAAAAAAAAADYnV5AAAAAAIAAAAAAAAABWJ1eWVyAAAAAAAAEwAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAA=",
        "AAAAAAAAAAAAAAALY2xhaW1fc2hhcmUAAAAAAgAAAAAAAAAHYWNjb3VudAAAAAATAAAAAAAAAAlzZXJpZXNfaWQAAAAAAAAKAAAAAA==",
        "AAAAAAAAAAAAAAAHdXBncmFkZQAAAAABAAAAAAAAAA1uZXdfd2FzbV9oYXNoAAAAAAAD7gAAACAAAAAA",
        "AAAAAgAAAAAAAAAAAAAAB0RhdGFLZXkAAAAAEAAAAAAAAAAAAAAABUFkbWluAAAAAAAAAAAAAAAAAAAETmFtZQAAAAAAAAAAAAAABlN5bWJvbAAAAAAAAQAAAAAAAAAITWV0YWRhdGEAAAABAAAACgAAAAEAAAAAAAAABU93bmVyAAAAAAAAAQAAAAoAAAABAAAAAAAAAAVUb2tlbgAAAAAAAAEAAAAKAAAAAQAAAAAAAAAFUHJpY2UAAAAAAAABAAAACgAAAAAAAAAAAAAAC05hdGl2ZVRva2VuAAAAAAAAAAAAAAAABlNlcmllcwAAAAAAAQAAAAAAAAALU2VyaWVzU2FsZXMAAAAAAQAAAAoAAAABAAAAAAAAAAxGYW5CYXNlUHJpY2UAAAABAAAACgAAAAEAAAAAAAAADEZhbkRlY2F5UmF0ZQAAAAEAAAAKAAAAAQAAAAAAAAAJU3VtRmFuQ3V0AAAAAAAAAQAAAAoAAAABAAAAAAAAAAZGYW5DdXQAAAAAAAIAAAAKAAAACgAAAAAAAAAAAAAABlN1cHBseQAAAAAAAAAAAAAAAAAEV2FzbQ==",
        "AAAAAgAAAAAAAAAAAAAAC1VzZXJEYXRhS2V5AAAAAAgAAAABAAAAAAAAAAdDcmVhdG9yAAAAAAEAAAAKAAAAAQAAAAAAAAANQ3JlYXRvckN1cnZlZAAAAAAAAAEAAAAKAAAAAQAAAAAAAAAKVG9rZW5Pd25lcgAAAAAAAQAAAAoAAAABAAAAAAAAAAtPd25lZFRva2VucwAAAAABAAAAEwAAAAEAAAAAAAAAEE93bmVkU2VyaWVzT3JkZXIAAAACAAAAEwAAAAoAAAABAAAAAAAAAAlMYXN0Q2xhaW0AAAAAAAACAAAAEwAAAAoAAAABAAAAAAAAAAdCYWxhbmNlAAAAAAEAAAATAAAAAQAAAAAAAAANU2VyaWVzQmFsYW5jZQAAAAAAAAIAAAATAAAACg==",
        "AAAAAQAAAAAAAAAAAAAAC0NyZWF0ZUV2ZW50AAAAAAcAAAAAAAAACmJhc2VfcHJpY2UAAAAAAAoAAAAAAAAAB2NyZWF0b3IAAAAAEwAAAAAAAAANY3JlYXRvcl9jdXJ2ZQAAAAAAAAoAAAAAAAAADmZhbl9iYXNlX3ByaWNlAAAAAAAKAAAAAAAAAA5mYW5fZGVjYXlfcmF0ZQAAAAAACgAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAAAAAADdXJpAAAAABA=",
        "AAAAAQAAAAAAAAAAAAAACEJ1eUV2ZW50AAAABgAAAAAAAAAFYnV5ZXIAAAAAAAATAAAAAAAAAAtjcmVhdG9yX2N1dAAAAAAKAAAAAAAAAAdmYW5fY3V0AAAAAAoAAAAAAAAABXByaWNlAAAAAAAACgAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAAAAAAIdG9rZW5faWQAAAAK",
        "AAAAAQAAAAAAAAAAAAAACkNsYWltRXZlbnQAAAAAAAQAAAAAAAAABmFtb3VudAAAAAAACgAAAAAAAAAObGFzdF93aXRoZHJhd24AAAAAAAoAAAAAAAAABW93bmVyAAAAAAAAEwAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACg==",
        "AAAAAQAAAAAAAAAAAAAACE1ldGFkYXRhAAAAAwAAAAAAAAANZGF0YV9maWxlX3VyaQAAAAAAABAAAAAAAAAABmlzc3VlcgAAAAAAEwAAAAAAAAAGc3ltYm9sAAAAAAAQ",
        "AAAAAQAAAAAAAAAAAAAABlNlcmllcwAAAAAABgAAAAAAAAAKYXJ0aXN0X2N1dAAAAAAACgAAAAAAAAAHY3JlYXRvcgAAAAATAAAAAAAAAAdmYW5fY3V0AAAAAAoAAAAAAAAACG1ldGFkYXRhAAAH0AAAAAhNZXRhZGF0YQAAAAAAAAAFcHJpY2UAAAAAAAAKAAAAAAAAAAVzYWxlcwAAAAAAAAo=",
        "AAAAAAAAACFSZXR1cm5zIHRoZSBvd25lciBvZiB0aGUgY29udHJhY3QAAAAAAAAJb3duZXJfZ2V0AAAAAAAAAAAAAAEAAAPoAAAAEw==",
        "AAAAAAAAAGhTZXRzIHRoZSBvd25lciBvZiB0aGUgY29udHJhY3QuIElmIG9uZSBhbHJlYWR5IHNldCBpdCB0cmFuc2ZlcnMgaXQgdG8gdGhlIG5ldyBvd25lciwgaWYgc2lnbmVkIGJ5IG93bmVyLgAAAAlvd25lcl9zZXQAAAAAAAABAAAAAAAAAAluZXdfb3duZXIAAAAAAAATAAAAAA==",
        "AAAAAAAAACRSZWRlcGxveSB0aGUgY29udHJhY3QgdG8gYSBXYXNtIGhhc2gAAAAIcmVkZXBsb3kAAAABAAAAAAAAAAl3YXNtX2hhc2gAAAAAAAPuAAAAIAAAAAA="
        ]);
    }
    private readonly parsers = {
        initialize: () => {},
        admin: (result: XDR_BASE64): string => this.spec.funcResToNative("admin", result),
        numberOfSeries: (result: XDR_BASE64): u128 => this.spec.funcResToNative("number_of_series", result),
        createSeries: () => {},
        seriesInfo: (result: XDR_BASE64): Series => this.spec.funcResToNative("series_info", result),
        seriesSales: (result: XDR_BASE64): u128 => this.spec.funcResToNative("series_sales", result),
        creatorOf: (result: XDR_BASE64): string => this.spec.funcResToNative("creator_of", result),
        shareBalance: (result: XDR_BASE64): u128 => this.spec.funcResToNative("share_balance", result),
        buy: () => {},
        claimShare: () => {},
        upgrade: () => {},
        ownerGet: (result: XDR_BASE64): Option<string> => this.spec.funcResToNative("owner_get", result),
        ownerSet: () => {},
        redeploy: () => {}
    };
    private txFromJSON = <T>(json: string): AssembledTransaction<T> => {
        const { method, ...tx } = JSON.parse(json)
        return AssembledTransaction.fromJSON(
            {
                ...this.options,
                method,
                parseResultXdr: this.parsers[method],
            },
            tx,
        );
    }
    public readonly fromJSON = {
        initialize: this.txFromJSON<ReturnType<typeof this.parsers['initialize']>>,
        admin: this.txFromJSON<ReturnType<typeof this.parsers['admin']>>,
        numberOfSeries: this.txFromJSON<ReturnType<typeof this.parsers['numberOfSeries']>>,
        createSeries: this.txFromJSON<ReturnType<typeof this.parsers['createSeries']>>,
        seriesInfo: this.txFromJSON<ReturnType<typeof this.parsers['seriesInfo']>>,
        seriesSales: this.txFromJSON<ReturnType<typeof this.parsers['seriesSales']>>,
        creatorOf: this.txFromJSON<ReturnType<typeof this.parsers['creatorOf']>>,
        shareBalance: this.txFromJSON<ReturnType<typeof this.parsers['shareBalance']>>,
        buy: this.txFromJSON<ReturnType<typeof this.parsers['buy']>>,
        claimShare: this.txFromJSON<ReturnType<typeof this.parsers['claimShare']>>,
        upgrade: this.txFromJSON<ReturnType<typeof this.parsers['upgrade']>>,
        ownerGet: this.txFromJSON<ReturnType<typeof this.parsers['ownerGet']>>,
        ownerSet: this.txFromJSON<ReturnType<typeof this.parsers['ownerSet']>>,
        redeploy: this.txFromJSON<ReturnType<typeof this.parsers['redeploy']>>
    }
        /**
    * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    initialize = async ({admin, native_token}: {admin: string, native_token: string}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'initialize',
            args: this.spec.funcArgsToScVals("initialize", {admin: new Address(admin), native_token: new Address(native_token)}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['initialize'],
        });
    }


        /**
    * Construct and simulate a admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    admin = async (options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'admin',
            args: this.spec.funcArgsToScVals("admin", {}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['admin'],
        });
    }


        /**
    * Construct and simulate a number_of_series transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    numberOfSeries = async (options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'number_of_series',
            args: this.spec.funcArgsToScVals("number_of_series", {}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['numberOfSeries'],
        });
    }


        /**
    * Construct and simulate a create_series transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    createSeries = async ({creator, uri, base_price, creator_curve, fan_base_price, fan_decay_rate}: {creator: string, uri: string, base_price: u128, creator_curve: u128, fan_base_price: u128, fan_decay_rate: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'create_series',
            args: this.spec.funcArgsToScVals("create_series", {creator: new Address(creator), uri, base_price, creator_curve, fan_base_price, fan_decay_rate}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['createSeries'],
        });
    }


        /**
    * Construct and simulate a series_info transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    seriesInfo = async ({series_id}: {series_id: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'series_info',
            args: this.spec.funcArgsToScVals("series_info", {series_id}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['seriesInfo'],
        });
    }


        /**
    * Construct and simulate a series_sales transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    seriesSales = async ({series_id}: {series_id: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'series_sales',
            args: this.spec.funcArgsToScVals("series_sales", {series_id}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['seriesSales'],
        });
    }


        /**
    * Construct and simulate a creator_of transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    creatorOf = async ({series_id}: {series_id: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'creator_of',
            args: this.spec.funcArgsToScVals("creator_of", {series_id}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['creatorOf'],
        });
    }


        /**
    * Construct and simulate a share_balance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    shareBalance = async ({account, series_id}: {account: string, series_id: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'share_balance',
            args: this.spec.funcArgsToScVals("share_balance", {account: new Address(account), series_id}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['shareBalance'],
        });
    }


        /**
    * Construct and simulate a buy transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    buy = async ({buyer, series_id}: {buyer: string, series_id: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'buy',
            args: this.spec.funcArgsToScVals("buy", {buyer: new Address(buyer), series_id}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['buy'],
        });
    }


        /**
    * Construct and simulate a claim_share transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    claimShare = async ({account, series_id}: {account: string, series_id: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'claim_share',
            args: this.spec.funcArgsToScVals("claim_share", {account: new Address(account), series_id}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['claimShare'],
        });
    }


        /**
    * Construct and simulate a upgrade transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    upgrade = async ({new_wasm_hash}: {new_wasm_hash: Buffer}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'upgrade',
            args: this.spec.funcArgsToScVals("upgrade", {new_wasm_hash}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['upgrade'],
        });
    }


        /**
    * Construct and simulate a owner_get transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.Returns the owner of the contract
    */
    ownerGet = async (options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'owner_get',
            args: this.spec.funcArgsToScVals("owner_get", {}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['ownerGet'],
        });
    }


        /**
    * Construct and simulate a owner_set transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.Sets the owner of the contract. If one already set it transfers it to the new owner, if signed by owner.
    */
    ownerSet = async ({new_owner}: {new_owner: string}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'owner_set',
            args: this.spec.funcArgsToScVals("owner_set", {new_owner: new Address(new_owner)}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['ownerSet'],
        });
    }


        /**
    * Construct and simulate a redeploy transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.Redeploy the contract to a Wasm hash
    */
    redeploy = async ({wasm_hash}: {wasm_hash: Buffer}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'redeploy',
            args: this.spec.funcArgsToScVals("redeploy", {wasm_hash}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['redeploy'],
        });
    }

}