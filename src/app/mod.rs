

pub mod web;
pub mod utils;


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
        self.web_server.start().await
    }
}