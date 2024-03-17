use serde::Deserialize;
use once_cell::sync::Lazy;
use std::fs;
use std::sync::RwLock;

#[derive(Debug)]
#[derive(Deserialize)]
pub struct Config {
   pub ip: String,
   pub port: Option<u16>,
   pub keys: Keys,
   pub settings: Settings,
}
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Keys {
    pub github: String,
    pub travis: Option<String>,
}
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Settings {
    pub trade_open_strategy: String,
    pub quote_coin: String,
    pub coin_pairs: String,
    pub total_money_per_symbol: u32,
    pub max_symbol_order_count_per_day: u32,
    pub sleep_seconds: u32,
    pub magic_9_buy_signal_min_num: String,
    pub magic_9_default_stop_loss_percent: f32,
    pub magic_9_stop_loss_percent: String,
    pub magic_9_watch_remain_times: String,
    pub bolling_param: u32,
    pub symtem_open_auto_coins_picker: bool,
    pub auto_coins_picker_push_picker_result: bool,
    pub auto_coins_picker_top: u32,
    pub auto_coins_picker_min_quote_volume_24h: u32,
    pub auto_coins_picker_min_price_increase_1_month: f32,
    pub auto_coins_picker_max_price_increase_1_month: f32,
    pub auto_coins_picker_pick_strategy: String,
    pub auto_coins_picker_money_per_order: u32,
    pub magic_9_open_5_min_buy_signal: bool,
    pub magic_9_open_5_min_sell_signal: bool,
    pub grid_money_per_order: u32,
    pub grid_order_book_max_length: String,
}

static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    let content = fs::read_to_string("config.toml").expect("config.toml read fail");
    let config: Config = toml::from_str(&content).unwrap();
    
    RwLock::new(config)
});

pub fn config() -> std::sync::RwLockReadGuard<'static, Config> {
    CONFIG.read().unwrap()
}