use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;
use crate::app::config::config;
use crate::app::trading::find;
mod monitor;

async fn job12h() -> Result<bool> {
    info!("开始-12h-选股任务");
    let symtem_open_auto_coins_picker = config().settings.symtem_open_auto_coins_picker;
    let pick_strategy = config().settings.auto_coins_picker_pick_strategy.to_string();
    if symtem_open_auto_coins_picker && pick_strategy == "ma_up_trend_picker" {
        match find::find_ma_up_trend_coins().await {
            Ok(r) => {
                info!("{}", r);
            },
            Err(e) => {
                info!("{}", e);
            }
        }
    }

    Ok(true)
}

async fn job24h() -> Result<bool> {
    info!("job24h mock");
    
    Ok(true)
}

async fn job1h() -> Result<bool> {
    info!("job1h mock");

    Ok(true)
}

pub fn scheduler_start() {
    // loop, 每秒 一次
    monitor::monitor();

    tokio::spawn(async move {
        let mut last_exeute_12h_time :u128 = 0;
        let mut last_exeute_24h_time :u128 = 0;
        let mut last_exeute_1h_time :u128 = 0;
        
        loop {
            // info!("scheduler loop");
            let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("get duration_since_epoch fail");
            let now_ms = duration_since_epoch.as_millis();
            if now_ms - last_exeute_1h_time > 1000 * 3600 {
                let f = job1h();
                match f.await {
                    Ok(_) => {},
                    Err(e) => {
                        error!("err {}", e)
                    },
                }
                last_exeute_1h_time = now_ms;
            }
            if now_ms - last_exeute_12h_time > 1000 * 12 * 3600 {
                let f = job12h();
                match f.await {
                    Ok(_) => {},
                    Err(e) => {
                        error!("err {}", e)
                    },
                }
                last_exeute_12h_time = now_ms;
            }
            if now_ms - last_exeute_24h_time > 1000 * 24 * 3600 {
                let f = job24h();
                match f.await {
                    Ok(_) => {},
                    Err(e) => {
                        error!("err {}", e)
                    },
                }
                last_exeute_24h_time = now_ms;
            }

            thread::sleep(Duration::from_millis(500));
        }
    });
    // find_coins::find_breakout_coins();
}