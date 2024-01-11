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
        contractId: "CCJNBVS5R5PBALF3IHI4UTBSAME7EU4IKHOMOBNSJDWG3LXNPCVSLFNJ",
    }
} as const

/**
    
    */
export type DataKey = {tag: "Admin", values: void} | {tag: "Name", values: void} | {tag: "Symbol", values: void} | {tag: "Metadata", values: readonly [u128]} | {tag: "Owner", values: readonly [u128]} | {tag: "Token", values: readonly [u128]} | {tag: "Price", values: readonly [u128]} | {tag: "NativeToken", values: void} | {tag: "Series", values: void} | {tag: "SeriesSales", values: readonly [u128]} | {tag: "Fans", values: readonly [u128]} | {tag: "Supply", values: void};

/**
    
    */
export type UserDataKey = {tag: "Creator", values: readonly [u128]} | {tag: "TokenOwner", values: readonly [u128]} | {tag: "OwnedTokens", values: readonly [string]} | {tag: "Balance", values: readonly [string]} | {tag: "SeriesBalance", values: readonly [string]} | {tag: "Share", values: readonly [string]};

/**
    
    */
export interface Metadata {
  /**
    
    */
data_file_uri: string;
  /**
    
    */
long_description_uri: string;
  /**
    
    */
short_description_uri: string;
}

/**
    
    */
export interface Series {
  /**
    
    */
creator: string;
  /**
    
    */
metadata: Metadata;
  /**
    
    */
price: u128;
}

/**
    
    */
export const Errors = {

}

export class Contract {
    spec: ContractSpec;
    constructor(public readonly options: ClassOptions) {
        this.spec = new ContractSpec([
            "AAAAAAAAAAAAAAAKaW5pdGlhbGl6ZQAAAAAABAAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAAARuYW1lAAAAEAAAAAAAAAAGc3ltYm9sAAAAAAAQAAAAAAAAAAxuYXRpdmVfdG9rZW4AAAATAAAAAA==",
        "AAAAAAAAAAAAAAAFYWRtaW4AAAAAAAAAAAAAAQAAABM=",
        "AAAAAAAAAAAAAAAEbmFtZQAAAAAAAAABAAAAEA==",
        "AAAAAAAAAAAAAAAGc3ltYm9sAAAAAAAAAAAAAQAAABA=",
        "AAAAAAAAAAAAAAAIZGVjaW1hbHMAAAAAAAAAAQAAAAo=",
        "AAAAAAAAAAAAAAAMZ2V0X21ldGFkYXRhAAAAAQAAAAAAAAAIdG9rZW5faWQAAAAKAAAAAQAAB9AAAAAITWV0YWRhdGE=",
        "AAAAAAAAAAAAAAAGc3VwcGx5AAAAAAAAAAAAAQAAAAo=",
        "AAAAAAAAAAAAAAAQbnVtYmVyX29mX3NlcmllcwAAAAAAAAABAAAACg==",
        "AAAAAAAAAAAAAAANY3JlYXRlX3NlcmllcwAAAAAAAAMAAAAAAAAAB2NyZWF0b3IAAAAAEwAAAAAAAAADdXJpAAAAABAAAAAAAAAACmJhc2VfcHJpY2UAAAAAAAoAAAAA",
        "AAAAAAAAAAAAAAALc2VyaWVzX2luZm8AAAAAAQAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAEAAAfQAAAABlNlcmllcwAA",
        "AAAAAAAAAAAAAAAMc2VyaWVzX3NhbGVzAAAAAQAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAEAAAAK",
        "AAAAAAAAAAAAAAAKY3JlYXRvcl9vZgAAAAAAAQAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAEAAAAT",
        "AAAAAAAAAAAAAAAHYmFsYW5jZQAAAAABAAAAAAAAAAdhY2NvdW50AAAAABMAAAABAAAACg==",
        "AAAAAAAAAAAAAAAMb3duZWRfdG9rZW5zAAAAAQAAAAAAAAAHYWNjb3VudAAAAAATAAAAAQAAA+oAAAAK",
        "AAAAAAAAAAAAAAANc2hhcmVfYmFsYW5jZQAAAAAAAAEAAAAAAAAAB2FjY291bnQAAAAAEwAAAAEAAAAK",
        "AAAAAAAAAAAAAAAIdHJhbnNmZXIAAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAACdG8AAAAAABMAAAAAAAAAAmlkAAAAAAAKAAAAAA==",
        "AAAAAAAAAAAAAAANdHJhbnNmZXJfZnJvbQAAAAAAAAMAAAAAAAAABGZyb20AAAATAAAAAAAAAAJ0bwAAAAAAEwAAAAAAAAACaWQAAAAAAAoAAAAA",
        "AAAAAAAAAAAAAAADYnV5AAAAAAIAAAAAAAAABWJ1eWVyAAAAAAAAEwAAAAAAAAAJc2VyaWVzX2lkAAAAAAAACgAAAAA=",
        "AAAAAAAAAAAAAAAFb3duZXIAAAAAAAABAAAAAAAAAAh0b2tlbl9pZAAAAAoAAAABAAAAEw==",
        "AAAAAAAAAAAAAAALY2xhaW1fc2hhcmUAAAAAAQAAAAAAAAAHYWNjb3VudAAAAAATAAAAAA==",
        "AAAAAgAAAAAAAAAAAAAAB0RhdGFLZXkAAAAADAAAAAAAAAAAAAAABUFkbWluAAAAAAAAAAAAAAAAAAAETmFtZQAAAAAAAAAAAAAABlN5bWJvbAAAAAAAAQAAAAAAAAAITWV0YWRhdGEAAAABAAAACgAAAAEAAAAAAAAABU93bmVyAAAAAAAAAQAAAAoAAAABAAAAAAAAAAVUb2tlbgAAAAAAAAEAAAAKAAAAAQAAAAAAAAAFUHJpY2UAAAAAAAABAAAACgAAAAAAAAAAAAAAC05hdGl2ZVRva2VuAAAAAAAAAAAAAAAABlNlcmllcwAAAAAAAQAAAAAAAAALU2VyaWVzU2FsZXMAAAAAAQAAAAoAAAABAAAAAAAAAARGYW5zAAAAAQAAAAoAAAAAAAAAAAAAAAZTdXBwbHkAAA==",
        "AAAAAgAAAAAAAAAAAAAAC1VzZXJEYXRhS2V5AAAAAAYAAAABAAAAAAAAAAdDcmVhdG9yAAAAAAEAAAAKAAAAAQAAAAAAAAAKVG9rZW5Pd25lcgAAAAAAAQAAAAoAAAABAAAAAAAAAAtPd25lZFRva2VucwAAAAABAAAAEwAAAAEAAAAAAAAAB0JhbGFuY2UAAAAAAQAAABMAAAABAAAAAAAAAA1TZXJpZXNCYWxhbmNlAAAAAAAAAQAAABMAAAABAAAAAAAAAAVTaGFyZQAAAAAAAAEAAAAT",
        "AAAAAQAAAAAAAAAAAAAACE1ldGFkYXRhAAAAAwAAAAAAAAANZGF0YV9maWxlX3VyaQAAAAAAABAAAAAAAAAAFGxvbmdfZGVzY3JpcHRpb25fdXJpAAAAEAAAAAAAAAAVc2hvcnRfZGVzY3JpcHRpb25fdXJpAAAAAAAAEA==",
        "AAAAAQAAAAAAAAAAAAAABlNlcmllcwAAAAAAAwAAAAAAAAAHY3JlYXRvcgAAAAATAAAAAAAAAAhtZXRhZGF0YQAAB9AAAAAITWV0YWRhdGEAAAAAAAAABXByaWNlAAAAAAAACg=="
        ]);
    }
    private readonly parsers = {
        initialize: () => {},
        admin: (result: XDR_BASE64): string => this.spec.funcResToNative("admin", result),
        name: (result: XDR_BASE64): string => this.spec.funcResToNative("name", result),
        symbol: (result: XDR_BASE64): string => this.spec.funcResToNative("symbol", result),
        decimals: (result: XDR_BASE64): u128 => this.spec.funcResToNative("decimals", result),
        getMetadata: (result: XDR_BASE64): Metadata => this.spec.funcResToNative("get_metadata", result),
        supply: (result: XDR_BASE64): u128 => this.spec.funcResToNative("supply", result),
        numberOfSeries: (result: XDR_BASE64): u128 => this.spec.funcResToNative("number_of_series", result),
        createSeries: () => {},
        seriesInfo: (result: XDR_BASE64): Series => this.spec.funcResToNative("series_info", result),
        seriesSales: (result: XDR_BASE64): u128 => this.spec.funcResToNative("series_sales", result),
        creatorOf: (result: XDR_BASE64): string => this.spec.funcResToNative("creator_of", result),
        balance: (result: XDR_BASE64): u128 => this.spec.funcResToNative("balance", result),
        ownedTokens: (result: XDR_BASE64): Array<u128> => this.spec.funcResToNative("owned_tokens", result),
        shareBalance: (result: XDR_BASE64): u128 => this.spec.funcResToNative("share_balance", result),
        transfer: () => {},
        transferFrom: () => {},
        buy: () => {},
        owner: (result: XDR_BASE64): string => this.spec.funcResToNative("owner", result),
        claimShare: () => {}
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
        name: this.txFromJSON<ReturnType<typeof this.parsers['name']>>,
        symbol: this.txFromJSON<ReturnType<typeof this.parsers['symbol']>>,
        decimals: this.txFromJSON<ReturnType<typeof this.parsers['decimals']>>,
        getMetadata: this.txFromJSON<ReturnType<typeof this.parsers['getMetadata']>>,
        supply: this.txFromJSON<ReturnType<typeof this.parsers['supply']>>,
        numberOfSeries: this.txFromJSON<ReturnType<typeof this.parsers['numberOfSeries']>>,
        createSeries: this.txFromJSON<ReturnType<typeof this.parsers['createSeries']>>,
        seriesInfo: this.txFromJSON<ReturnType<typeof this.parsers['seriesInfo']>>,
        seriesSales: this.txFromJSON<ReturnType<typeof this.parsers['seriesSales']>>,
        creatorOf: this.txFromJSON<ReturnType<typeof this.parsers['creatorOf']>>,
        balance: this.txFromJSON<ReturnType<typeof this.parsers['balance']>>,
        ownedTokens: this.txFromJSON<ReturnType<typeof this.parsers['ownedTokens']>>,
        shareBalance: this.txFromJSON<ReturnType<typeof this.parsers['shareBalance']>>,
        transfer: this.txFromJSON<ReturnType<typeof this.parsers['transfer']>>,
        transferFrom: this.txFromJSON<ReturnType<typeof this.parsers['transferFrom']>>,
        buy: this.txFromJSON<ReturnType<typeof this.parsers['buy']>>,
        owner: this.txFromJSON<ReturnType<typeof this.parsers['owner']>>,
        claimShare: this.txFromJSON<ReturnType<typeof this.parsers['claimShare']>>
    }
        /**
    * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    initialize = async ({admin, name, symbol, native_token}: {admin: string, name: string, symbol: string, native_token: string}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'initialize',
            args: this.spec.funcArgsToScVals("initialize", {admin: new Address(admin), name, symbol, native_token: new Address(native_token)}),
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
    * Construct and simulate a name transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    name = async (options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'name',
            args: this.spec.funcArgsToScVals("name", {}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['name'],
        });
    }


        /**
    * Construct and simulate a symbol transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    symbol = async (options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'symbol',
            args: this.spec.funcArgsToScVals("symbol", {}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['symbol'],
        });
    }


        /**
    * Construct and simulate a decimals transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    decimals = async (options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'decimals',
            args: this.spec.funcArgsToScVals("decimals", {}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['decimals'],
        });
    }


        /**
    * Construct and simulate a get_metadata transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    getMetadata = async ({token_id}: {token_id: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'get_metadata',
            args: this.spec.funcArgsToScVals("get_metadata", {token_id}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['getMetadata'],
        });
    }


        /**
    * Construct and simulate a supply transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    supply = async (options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'supply',
            args: this.spec.funcArgsToScVals("supply", {}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['supply'],
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
    createSeries = async ({creator, uri, base_price}: {creator: string, uri: string, base_price: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'create_series',
            args: this.spec.funcArgsToScVals("create_series", {creator: new Address(creator), uri, base_price}),
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
    * Construct and simulate a balance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    balance = async ({account}: {account: string}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'balance',
            args: this.spec.funcArgsToScVals("balance", {account: new Address(account)}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['balance'],
        });
    }


        /**
    * Construct and simulate a owned_tokens transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    ownedTokens = async ({account}: {account: string}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'owned_tokens',
            args: this.spec.funcArgsToScVals("owned_tokens", {account: new Address(account)}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['ownedTokens'],
        });
    }


        /**
    * Construct and simulate a share_balance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    shareBalance = async ({account}: {account: string}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'share_balance',
            args: this.spec.funcArgsToScVals("share_balance", {account: new Address(account)}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['shareBalance'],
        });
    }


        /**
    * Construct and simulate a transfer transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    transfer = async ({from, to, id}: {from: string, to: string, id: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'transfer',
            args: this.spec.funcArgsToScVals("transfer", {from: new Address(from), to: new Address(to), id}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['transfer'],
        });
    }


        /**
    * Construct and simulate a transfer_from transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    transferFrom = async ({from, to, id}: {from: string, to: string, id: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'transfer_from',
            args: this.spec.funcArgsToScVals("transfer_from", {from: new Address(from), to: new Address(to), id}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['transferFrom'],
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
    * Construct and simulate a owner transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    owner = async ({token_id}: {token_id: u128}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'owner',
            args: this.spec.funcArgsToScVals("owner", {token_id}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['owner'],
        });
    }


        /**
    * Construct and simulate a claim_share transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
    */
    claimShare = async ({account}: {account: string}, options: {
        /**
         * The fee to pay for the transaction. Default: 100.
         */
        fee?: number,
    } = {}) => {
        return await AssembledTransaction.fromSimulation({
            method: 'claim_share',
            args: this.spec.funcArgsToScVals("claim_share", {account: new Address(account)}),
            ...options,
            ...this.options,
            errorTypes: Errors,
            parseResultXdr: this.parsers['claimShare'],
        });
    }

}