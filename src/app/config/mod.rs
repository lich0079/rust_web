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
    pub lark_bot_webhook: String,
    pub lark_bot_webhook_1w: String,
    pub lark_bot_webhook_1d: String,
    pub lark_bot_webhook_4h: String,
}

static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    let content = fs::read_to_string("config.toml").expect("config.toml read fail");
    let config: Config = toml::from_str(&content).unwrap();
    
    RwLock::new(config)
});

pub fn config() -> std::sync::RwLockReadGuard<'static, Config> {
    CONFIG.read().unwrap()
}