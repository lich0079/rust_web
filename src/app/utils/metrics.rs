use std::sync::RwLock;

use actix_web_prom::{PrometheusMetricsBuilder, PrometheusMetrics};
use once_cell::sync::Lazy;


pub static PROMETHEUS: Lazy<RwLock<PrometheusMetrics>> = Lazy::new(|| {
    let prometheus: PrometheusMetrics = PrometheusMetricsBuilder::new("")
    .endpoint("/metrics")
    .build()
    .unwrap();
    
    RwLock::new(prometheus)
});
