use rust_decimal::prelude::*;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crypto_botters::{
    Client,
    binance::{BinanceHttpUrl, BinanceOption},
};
use std::thread;

// use async_std::task::block_on;

    // typed
    #[derive(Serialize)]
    struct TickerParams<'a> {
        symbol: &'a str,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Ticker {
        #[serde(with = "rust_decimal::serde::str")]
        price: Decimal,
        symbol: String,
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct Symbol {
        symbol: String,
        quoteAsset: String,
        status: String,
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    struct ExchangeInfo {
        #[allow(dead_code)]
        symbols: Vec<Symbol>,
    }

pub fn find_coins() {
    let mut client = Client::new();
    client.update_default_option(BinanceOption::HttpUrl(BinanceHttpUrl::Spot));

    tokio::spawn(async move {
        loop {
            thread::sleep(Duration::from_secs(1));
            // let ticker: Ticker = client.get(
            //     "/api/v3/ticker/price",
            //     Some(&TickerParams { symbol: "BTCUSDT" }),
            //     [BinanceOption::Default],
            // ).await.expect("failed to get tickers");
            // info!("BTC price: {}", ticker.price);

            // let ticker: Ticker = client.get(
            //     "/api/v3/ticker/price",
            //     Some(&TickerParams { symbol: "ETHUSDT" }),
            //     [BinanceOption::Default],
            // ).await.expect("failed to get tickers");
            // info!("ETH price: {}", ticker.price);

            let exchange_info: ExchangeInfo = client.get(
                "/api/v3/exchangeInfo",
                Some(&json!({ "permissions": "SPOT" })),
                [BinanceOption::Default],
            ).await.expect("failed to get tickers");
            let mut result = Vec::new();
            for symbol in exchange_info.symbols {
                if symbol.status == "TRADING" && symbol.quoteAsset == "USDT" {
                    result.push(symbol.symbol);
                }
            }
            println!("exchangeInfo :\n{:?}", result.len());
            let now = Instant::now();
            for symbol in &result {
                let klines: serde_json::Value = client.get(
                    "/api/v3/klines",
                    Some(&json!({ "symbol": symbol, "interval" : "1d", "limit" : "30" })),
                    [BinanceOption::Default],
                ).await.expect("failed to get klines");
                let arr = klines.as_array().unwrap();
                if arr.len() < 30 {
                    println!("klines not 30d {}", symbol);
                    continue;
                }
                let last_month_close : f32 = arr[0].as_array().unwrap()[4].as_str().unwrap().parse().unwrap();
                let current_close : f32 = arr[29].as_array().unwrap()[4].as_str().unwrap().parse().unwrap();
                let diff = current_close / last_month_close;
                if diff < 1.1 {
                    continue;
                }
                println!("{} diff:{} lastMonthClose:{} currentClose:{}", diff, symbol, last_month_close, current_close);
            }
            

            println!("cost {} ms", now.elapsed().as_millis());
        }
    });
}