mod config;
mod errors;
mod handlers;
mod models;

use std::sync::Mutex;

use actix_web::web;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use log::info;

use crate::config::Config;
use crate::models::Ticket;

struct AppState {
    tickets: Mutex<Vec<Ticket>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let config = Config::from_env().unwrap();

    let app_state = web::Data::new(AppState {
        tickets: Mutex::new(Vec::new()),
    });

    info!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(handlers::health)
            .service(handlers::post_ticket)
            .service(handlers::get_tickets)
            .service(handlers::get_ticket)
            .service(handlers::update_ticket)
            .service(handlers::delete_ticket)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
