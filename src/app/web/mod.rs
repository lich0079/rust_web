use actix_web::{get, web, App, HttpServer, Responder};
pub use crate::app::utils::metrics::PROMETHEUS;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

pub struct WebServer {
}


impl WebServer {
    pub fn new() -> Self {
        WebServer {}
    }

    pub async fn start(&self) -> Result<(), std::io::Error>{
        HttpServer::new(move || {
            App::new()
                .wrap(PROMETHEUS.read().unwrap().clone())
                .service(index)
                .service(hello)
        })
        .bind(("127.0.0.1", 8080)).unwrap()
        .run().await
    }
}