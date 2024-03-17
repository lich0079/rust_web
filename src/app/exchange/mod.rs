use serde::Deserialize;
use rust_decimal::prelude::*;


pub mod binance;

    // // typed
    // #[derive(Serialize)]
    // pub struct TickerParams<'a> {
    //     symbol: &'a str,
    // }

    // #[derive(Deserialize)]
    // #[allow(dead_code)]
    // pub struct Ticker {
    //     #[serde(with = "rust_decimal::serde::str")]
    //     price: Decimal,
    //     symbol: String,
    // }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    pub struct Symbol {
        symbol: String,
        quoteAsset: String,
        status: String,
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct ExchangeInfo {
        #[allow(dead_code)]
        symbols: Vec<Symbol>,
    }

//    [
//   [
//     1499040000000,      // Kline open time
//     "0.01634790",       // Open price
//     "0.80000000",       // High price
//     "0.01575800",       // Low price
//     "0.01577100",       // Close price
//     "148976.11427815",  // Volume
//     1499644799999,      // Kline Close time
//     "2434.19055334",    // Quote asset volume
//     308,                // Number of trades
//     "1756.87402397",    // Taker buy base asset volume
//     "28.46694368",      // Taker buy quote asset volume
//     "0"                 // Unused field, ignore.
//   ]
// ]
    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct Kline (
        u64,
        String,
        String,
        String,
        pub String,
        String,
        u64,
        String,
        u64,
        String,
        String,
        String
    );

    // {
    //     "symbol": "BTCUSDT",
    //     "priceChange": "-1293.72000000",
    //     "priceChangePercent": "-1.896",
    //     "weightedAvgPrice": "66567.60858008",
    //     "openPrice": "68250.00000000",
    //     "highPrice": "68499.10000000",
    //     "lowPrice": "64533.00000000",
    //     "lastPrice": "66956.28000000",
    //     "volume": "64620.87167000",
    //     "quoteVolume": "4301656891.43229610",
    //     "openTime": 1710590160000,
    //     "closeTime": 1710676576217,
    //     "firstId": 3488945658,
    //     "lastId": 3491891429,
    //     "count": 2945772
    // }
    #[derive(Debug)]
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    pub struct Ticker {
       pub symbol: String,
       #[serde(with = "rust_decimal::serde::str")]
       pub priceChange: Decimal,
       #[serde(with = "rust_decimal::serde::str")]
       pub priceChangePercent: Decimal,
       #[serde(with = "rust_decimal::serde::str")]
       pub weightedAvgPrice: Decimal,
       #[serde(with = "rust_decimal::serde::str")]
       pub openPrice: Decimal,
       #[serde(with = "rust_decimal::serde::str")]
       pub highPrice: Decimal,
       #[serde(with = "rust_decimal::serde::str")]
       pub lowPrice: Decimal,
       #[serde(with = "rust_decimal::serde::str")]
       pub lastPrice: Decimal,
       #[serde(with = "rust_decimal::serde::str")]
       pub volume: Decimal,
       #[serde(with = "rust_decimal::serde::str")]
       pub quoteVolume: Decimal,
       pub openTime: u128,
       pub closeTime: u128,
    }
