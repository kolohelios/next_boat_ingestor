use actix_web::{get, web, App, HttpServer, HttpResponse, error};
use deadpool_redis::{cmd, Pool, PoolError};
use deadpool_redis::redis::RedisError;
use dotenv::dotenv;
use serde::Deserialize;
use thiserror::*;
use chrono::{Utc};

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    redis: deadpool_redis::Config
}

impl Config {
    pub fn from_env() -> Result<Self, ::config_crate::ConfigError> {
        let mut cfg = ::config_crate::Config::new();
        cfg.merge(::config_crate::Environment::new().separator("__"))?;
        cfg.try_into()
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error("Pool error:`{0}`")]
    PoolError(#[from]PoolError),
    #[error("Redis error:`{0}`")]
    RedisError(#[from]RedisError),
}

impl error::ResponseError for Error {}

#[get("/")]
async fn index(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().await?;
    let key = String::from("test2");
    let value = 2848490;
    let now = Utc::now().timestamp();
    cmd("TS.CREATE").arg(&[&key]).execute_async(&mut conn).await.ok();
    let _ = cmd("TS.ADD").arg(&key).arg(now).arg(value).execute_async(&mut conn).await;
    let retrieved_value: (u64, String) = cmd("TS.GET").arg(&[&key]).query_async(&mut conn).await.unwrap();
    let new_value = retrieved_value.1;
    Ok(HttpResponse::Ok().body(format!("set {}:{}", key, new_value)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let cfg = Config::from_env().unwrap();
    let pool = cfg.redis.create_pool().unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(index)
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
