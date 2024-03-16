
use std::thread;

mod monitor;
mod find_coins;

pub fn scheduler_start() {
    thread::spawn(|| monitor::monitor());

    find_coins::find_coins();
}