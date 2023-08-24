use actix_web::{middleware, web, App, HttpServer};
use chrono::Local;
use dotenv::dotenv;
use log::info;
use routers::*;
use sea_orm::{ConnectOptions, Database};
use state::AppState;
use std::io::Write;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::{env, io};
use actix_web::dev::Service;
use futures_util::FutureExt;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../error.rs"]
mod error;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../middleware/mod.rs"]
mod middle;
#[path = "../models/mod.rs"]
mod models;
#[path = "../result.rs"]
mod result;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../middleware/mod.rs"]
mod my_middleware;

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
            // 日志中间件
            .wrap(middleware::Logger::default())
            //.wrap(request_logger) // 自定义的中间件
            .wrap_fn(|req, srv| {
                info!("1. Hi from start. You requested: {}", req.path());
                srv.call(req).map(|res| {
                    info!("2. Hi from response");
                    res
                })
            })
            .wrap_fn(|req, srv| {
                let start_time = Instant::now();
                let path = req.path().to_owned();
                println!("{}", "2. Pre-process the Request");
                let fut = srv.call(req);
                async move {
                    let res = fut.await;
                    let elapsed_time = start_time.elapsed();
                    println!("{}", "3. Post-process a Response");
                    println!(
                        "Request to {} took {:?}",
                        path,
                        elapsed_time
                    );
                    res
                }
            })
            .wrap(my_middleware::timedMiddleware::Timed)
            .wrap(my_middleware::auth::Auth)
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
    use env_logger::fmt::Color;
    use env_logger::Env;
    use log::LevelFilter;

    let env = Env::default().filter_or("MY_LOG_LEVEL", "debug");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug | log::Level::Trace => Color::Cyan,
            };

            let mut level_style = buf.style();
            level_style.set_color(level_color).set_bold(true);

            let mut style = buf.style();
            style.set_color(Color::White).set_dimmed(true);

            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_style.value(record.level()),
                style.value(record.module_path().unwrap_or("<unnamed>")),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();
    info!("env_logger initialized.");
}