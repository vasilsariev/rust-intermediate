#[macro_use]
extern crate actix_web;

use {
    actix_web::{App, HttpServer, middleware, web::Data},
    sqlx::postgres::PgPoolOptions,
    std::{env, io},
    tracing_actix_web::TracingLogger,
    tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt},
};

mod miner;
mod miner_controller;
mod util;
mod wallet;
mod wallet_controller;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let port = 9090;
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("actix_web=debug,actix_server=info,sqlx=warn")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_pool = env::var("DATABASE_URL").ok().and_then(|database_url| {
        PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy(&database_url)
            .ok()
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
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
