use std::{env, io};
use std::sync::Mutex;
use std::time::Duration;

use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use log::info;
use sea_orm::{ConnectOptions, Database};

use routers::*;
use state::AppState;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../error.rs"]
mod error;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../result.rs"]
mod result;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 读取环境变量
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    info!("database_url:[{}]", database_url);
    let mut opt = ConnectOptions::new(database_url);
    // 初始化数据库
    init_db(&mut opt);
    let db = Database::connect(opt).await.unwrap();
    // 初始化日志
    init_logger();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db,
    });
    let app = move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

fn init_db(opt: &mut ConnectOptions) {
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public"); // Setting default PostgreSQL schema
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
