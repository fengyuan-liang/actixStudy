use std::env;

use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use log::info;
use tera::Tera;

use crate::webapp::routers::app_config;

#[path = "../mod.rs"]
mod webapp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_port = env::var("HOST_PORT").expect("host port not found");
    info!("Listening on: {}", &host_port);
    // 初始化日志
    init_logger();
    // 打印static路径
    info!("static path {}", concat!(env!("CARGO_MANIFEST_DIR"), "./static/**/*"));
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "./static/**/*")).unwrap();
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(tera)).configure(app_config)
    })
        .bind(&host_port)?
        .run()
        .await
}


fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}