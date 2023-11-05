use actix_web::{get, web, App, HttpServer, Responder};
use actix_web_prom::PrometheusMetricsBuilder;
use prometheus::Gauge;
use systemstat::{Platform, System};
use dashmap::DashMap;

use std::thread;
use std::time::Duration;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sys = System::new();

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    let cpu_usage = Gauge::new("cpu_usage", "Current CPU usage in percent").unwrap();
    let mem_usage = Gauge::new("mem_usage", "Current memory usage in percent").unwrap();

    prometheus
        .registry
        .register(Box::new(mem_usage.clone()))
        .unwrap();
    prometheus
    .registry
    .register(Box::new(cpu_usage.clone()))
    .unwrap();
    thread::spawn(move || loop {
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
    });

    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .service(index)
            .service(hello)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
