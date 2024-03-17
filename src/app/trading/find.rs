use anyhow::Result;

use crate::app::exchange::binance;

pub async fn find_ma_up_trend_coins() -> Result<bool> {

    let symbols = binance::get_symbols().await?;
    info!("get_symbols size {}", symbols.len());

    for symbol in symbols {
        match binance::get_klines(&symbol).await {
            Ok(klines) => {
                // let klines = binance::get_klines(&symbol).await?;
                if klines.len() < 30 {
                    info!("klines not 30d {}", symbol);
                    continue;
                }
                let last_month_close : f32 = klines[0].4.parse()?;
                let current_close : f32 = klines[29].4.parse()?;
                let diff = current_close / last_month_close;
                if diff < 1.1 {
                    info!("{} 与30天前的价格相比,未突破 1.1 倍,不考虑, {}", symbol, diff);
                    continue;
                }
                if diff > 1.5 {
                    info!("{} 与30天前的价格相比,涨幅已经超过 1.5 倍,不考虑, {}", symbol, diff);
                    continue;
                }
            },
            Err(e) => {
                error!("get_klines error, {}", e)
            },
        };
        match binance::get_ticker(&symbol).await {
            Ok(ticker) => {
                info!("ticker {:?}", ticker)
            },
            Err(e) => {
                error!("get_ticker error, {}", e)
            },
        };
    }
    Ok(true)
}