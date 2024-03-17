

pub mod web;
pub mod utils;
pub mod schedule;
pub mod exchange;
pub mod config;
pub mod trading;


pub struct App {
    web_server : web::WebServer
}

impl App {
    pub fn new () -> Self {
        App {
            web_server : web::WebServer::new()
        }
    }

    pub async fn start(&self) -> Result<(), std::io::Error> {
        info!("app start");
        info!("config {:?}", config::config().settings);
        self.web_server.start().await
    }
}