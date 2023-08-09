use std::{env, io};
use std::sync::Mutex;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Service;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use routers::*;
use state::AppState;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../result.rs"]
mod result;
#[path = "../middleware.rs"]
mod middleware;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 读取环境变量
    dotenv.ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });
    let app = move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

