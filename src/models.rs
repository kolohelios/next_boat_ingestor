use actix_web::{error};
use deadpool_redis::redis::RedisError;
use deadpool_redis::{PoolError};
use serde::{Deserialize, Serialize};
use thiserror::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Pool error:`{0}`")]
    PoolError(#[from]PoolError),
    #[error("Redis error:`{0}`")]
    RedisError(#[from]RedisError),
}

impl error::ResponseError for Error {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Terminal {
    TerminalID: i8,
    Description: String
}
