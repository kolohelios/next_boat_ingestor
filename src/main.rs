use actix_web::{get, web, App, HttpServer, HttpResponse, error, client::Client};
use deadpool_redis::{cmd, Pool, PoolError};
use deadpool_redis::redis::RedisError;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use thiserror::*;
use chrono::{Utc};

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    redis: deadpool_redis::Config,
    wsf_api_key: String
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

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct Terminal {
    TerminalID: i8,
    Description: String
}

#[get("/")]
async fn index(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client = Client::default();

   // Create request builder and send request
    let cfg = Config::from_env().unwrap();
    let api_key = cfg.wsf_api_key;
    let response = client.get(format!("https://www.wsdot.wa.gov/ferries/api/schedule/rest/terminals/2022-06-02?apiaccesscode={}", api_key))
        .header("User-Agent", "actix-web/3.0")
        .send()
        .await
        .unwrap()
        .json::<Vec<Terminal>>()
        .await;

    println!("Response: {:?}", response);

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
