#[macro_use]
extern crate log;

mod app;

pub use app::*;

#[deny(clippy::cast_possible_truncation)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("booting up");

    info!("booting up {}", "可厉害".len());

    info!("booting up {}", "可厉害".chars().count());

    // 注册 Ctrl+C 信号处理器
    ctrlc::set_handler(move || {
        // 设置 running 变量为 false，表示程序应该退出
        info!("ctrlc::set_handler");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    app::schedule::scheduler_start();

    let app = app::App::new();
    app.start().await
}
