use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

// use actix_cors::Cors;

use std::env;
use dotenv::dotenv;

use env_logger::{Env, Builder, Target};
use log::{info, warn, error, log_enabled, Level, debug, trace};

use sqlx::postgres::PgPoolOptions;


// #[path = "../log.rs"]
// mod log;

#[path = "../db_access.rs"]
mod db_access;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../routers.rs"]
mod routers;

#[path = "../state.rs"]
mod state;

#[path = "../models.rs"]
mod models;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // --- 读取环境变量
    dotenv().ok();
    // 读取环境变量 ---

    // --- 载入日志
    // env_logger::init();
    // let mut builder = Builder::from_default_env();
    // builder.target(Target::Stdout);
    // builder.init();
    let env = Env::default()
    .filter_or("LOG_LEVEL", "info")         // (环境变量, 缺省)
    .write_style_or("LOG_STYLE", "always");  // (环境变量, 缺省)
    env_logger::init_from_env(env);
    // 载入日志 --- 

    // --- 初始化
    if dotenv().is_ok() {
        info!("Environment config Ok!");
    } else {
        error!("Environment config invalid: .env file not found");
    }
    debug!("This is a debug version!");
    // 初始化 ---

    // --- 载入数据库
    let database_url = env::var("DATABASE_URL");
    match &database_url {
        Ok(url) => info!("Connected to Database [{}]", url),
        Err(e) => error!("Failed to connect to database: [{}]", e),
    }
    let database_url = database_url.unwrap();
    
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    // 载入数据库 ---
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK".to_string(),
        visit_count: Mutex::new(0),

        db: db_pool,
        // animals: Mutex::new(vec![]),
        // login_data: vec![],
    });

    let app = move || {
        App::new()
        // --- 允许跨域 (release时不可用)
            // .wrap(Cors::default()
                // .allow_any_header()
                // .allow_any_method()
                // .allow_any_origin()
            // )
        // 允许跨域 ---
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(animal_routes)
    };

    info!("Server started at localhost:3000");

    HttpServer::new(app)
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use log::info;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn logger_works() {
        init();

        info!("This record will be captured by `cargo test`");

        assert_eq!(2, 1 + 1);
    }
}