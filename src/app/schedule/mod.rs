use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;
use crate::app::arh999;
use crate::app::trading::crossover;
mod monitor;

async fn job8h() -> Result<bool> {
    info!("job8h");

    match crossover::find_crossover("1d").await {
        Ok(r) => {
        },
        Err(e) => {
            error!("find_crossover 1d {}", e);
        }
    };

    Ok(true)
}

async fn job24h() -> Result<bool> {
    info!("job24h");
    match crossover::find_crossover("1w").await {
    Ok(r) => {
    },
    Err(e) => {
        error!("find_crossover 1w {}", e);
    }
    };

    let _ = arh999::get_index().await?;

    Ok(true)
}

async fn job1h() -> Result<bool> {
    info!("job1h");

    // match crossover::find_crossover("4h").await {
    //     Ok(r) => {
    //     },
    //     Err(e) => {
    //         error!("find_crossover 4h {}", e);
    //     }
    // };

    Ok(true)
}

pub fn scheduler_start() {
    // loop, 每秒 一次
    monitor::monitor();

    tokio::spawn(async move {
        let mut last_exeute_8h_time :u128 = 0;
        let mut last_exeute_24h_time :u128 = 0;
        let mut last_exeute_1h_time :u128 = 0;
        
        loop {
            let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("get duration_since_epoch fail");
            let now_ms = duration_since_epoch.as_millis();
            if now_ms - last_exeute_1h_time > 1000 * 4 * 3600 {
                let f = job1h();
                match f.await {
                    Ok(_) => {},
                    Err(e) => {
                        error!("job1h err {:?}", e)
                    },
                };
                last_exeute_1h_time = now_ms;
            }
            if now_ms - last_exeute_8h_time > 1000 * 8 * 3600 {
                let f = job8h();
                match f.await {
                    Ok(_) => {},
                    Err(e) => {
                        error!("job8h err {:?}", e)
                    },
                };
                last_exeute_8h_time = now_ms;
            }
            if now_ms - last_exeute_24h_time > 1000 * 12 * 3600 {
                let f = job24h();
                match f.await {
                    Ok(_) => {},
                    Err(e) => {
                        error!("job24h err {}", e)
                    },
                };
                last_exeute_24h_time = now_ms;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(60000)).await;
        }
    });
}