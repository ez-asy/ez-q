mod configs;
mod core;
mod error;
mod redis_utils;
mod routes;

use actix_web::{web::Data, App, HttpServer};

use anyhow::bail;
use configs::json_config::create_json_config;
use redis_utils::redis_pool::create_redis_pool;
use routes::{
    controllers::queue_controllers::{fetch_message, list_all_queues, post_message},
    middlewares::{logger::create_logger_mw, normalise::create_normalise_mw},
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let redis_pool = match create_redis_pool().await {
        Ok(pool) => pool,
        Err(err) => bail!("Could not create a redis client pool: {}", err),
    };
    let redis_pool_app_data = Data::new(redis_pool);

    Ok(HttpServer::new(move || {
        let json_config = create_json_config();
        let logger_mw = create_logger_mw();
        let normalise_mw = create_normalise_mw();

        App::new()
            .wrap(logger_mw)
            .wrap(normalise_mw)
            .app_data(json_config)
            .app_data(redis_pool_app_data.clone())
            .service(list_all_queues)
            .service(fetch_message)
            .service(post_message)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?)
}
