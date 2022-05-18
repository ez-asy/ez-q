mod configs;
mod core;
mod error;
mod redis_utils;
mod routes;

use std::env;

use actix_web::{web::Data, App, HttpServer};

use anyhow::bail;
use configs::json_config::create_json_config;
use redis_utils::redis_pool::create_redis_pool;
use routes::{
    controllers::{
        message_controllers::{complete_message, fail_message},
        queue_controllers::{fetch_message, list_all_queues, post_message},
    },
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

    let host_binding = match env::var("DOCKER_BUILD") {
        Ok(value) => {
            if value == "1" {
                "0.0.0.0:8080"
            } else {
                "127.0.0.1:8080"
            }
        }
        Err(_) => "127.0.0.1:8080",
    };

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
            .service(complete_message)
            .service(fail_message)
    })
    .bind(host_binding)?
    .run()
    .await?)
}
