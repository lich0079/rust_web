#[macro_use]
extern crate log;

use log4rs;

mod app;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("booting up");

    app::schedule::scheduler_start();

    let app = app::App::new();
    app.start().await
}
