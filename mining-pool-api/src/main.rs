#[macro_use]
extern crate actix_web;

use {
    actix_web::{App, HttpServer, middleware, web::Data},
    sqlx::postgres::PgPoolOptions,
    std::{env, io},
    tracing_actix_web::TracingLogger,
    tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt},
};

mod db;
mod miner;
mod miner_controller;
mod util;
mod wallet;
mod wallet_controller;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let port = 9090;

    let wallet_db = db::create_wallet_db();

    HttpServer::new(move || {
        let app_data = Data::new(wallet_db.clone());
        App::new()
            .app_data(app_data)
            .wrap(TracingLogger::default())
            .wrap(middleware::NormalizePath::trim())
            .service(wallet_controller::list_wallets)
            .service(wallet_controller::get_wallet)
            .service(wallet_controller::create_wallet)
            .service(miner_controller::list_miners)
            .service(miner_controller::get_miner)
            .service(miner_controller::create_miner)
    })
    .bind(("0.0.0.0", port))?
    .workers(2)
    .run()
    .await
}
