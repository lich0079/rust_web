use prometheus::Gauge;
use systemstat::{Platform, System};
use std::thread;
use std::time::Duration;
pub use crate::app::utils::metrics::PROMETHEUS;

pub fn monitor() {
    let sys = System::new();

    let cpu_usage = Gauge::new("cpu_usage", "Current CPU usage in percent").unwrap();
    let mem_usage = Gauge::new("mem_usage", "Current memory usage in percent").unwrap();

    PROMETHEUS.read().unwrap()
        .registry
        .register(Box::new(mem_usage.clone()))
        .unwrap();
    PROMETHEUS.read().unwrap()
    .registry
    .register(Box::new(cpu_usage.clone()))
    .unwrap();

    loop {
        thread::sleep(Duration::from_secs(1));
        match sys.memory() {
            Ok(mem) => {
                let memory_used = mem.total.0 - mem.free.0;
                let pourcentage_used = (memory_used as f64 / mem.total.0 as f64) * 100.0;
                mem_usage.set(pourcentage_used);
            }
            Err(x) => println!("\nMemory: error: {}", x),
        }
        match sys.load_average() {
            Ok(loadavg) => {
                cpu_usage.set(loadavg.one as f64);
            }
            Err(x) => println!("\nLoad average: error: {}", x)
        }
    }
}