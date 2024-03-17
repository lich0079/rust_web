
use anyhow::Result;
use super::ExchangeInfo;
use super::Kline;
use super::Ticker;

pub async fn get_symbols () -> Result<Vec<String>> {
    let body = reqwest::get("https://api.binance.com/api/v3/exchangeInfo")
    .await?
    .text()
    .await?;

    let exchange_info: ExchangeInfo = serde_json::from_str(&body)?;
    let mut result = Vec::new();
    for symbol in exchange_info.symbols {
        if symbol.status == "TRADING" && symbol.quoteAsset == "USDT" {
            result.push(symbol.symbol);
        }
    }
    Ok(result)
}

pub async fn get_klines (name : &str) -> Result<Vec<Kline>> {
    // "symbol": symbol, "interval" : "1d", "limit" : "30"
    let body = reqwest::get(format!("https://api.binance.com/api/v3/klines?symbol={}&interval=1d&limit=30", name))
    .await?
    .text()
    .await?;

    let klines: Vec<Kline> = serde_json::from_str(&body)?;

    Ok(klines)
}

pub async fn get_ticker (name : &str) -> Result<Ticker> {
    let body = reqwest::get(format!("https://api.binance.com/api/v3/ticker?symbol={}", name))
    .await?
    .text()
    .await?;

    info!("get_ticker {}", body);
    let ticker: Ticker = serde_json::from_str(&body)?;

    Ok(ticker)
}