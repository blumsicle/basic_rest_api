mod config;
mod handlers;
mod models;

use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use log::info;

use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let config = Config::from_env().unwrap();

    info!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(|| App::new().wrap(Logger::default()).service(handlers::status))
        .bind((config.server.host, config.server.port))?
        .run()
        .await
}
