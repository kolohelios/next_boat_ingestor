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
    TerminalID: u8,
    Description: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct VesselLocation {
    pub VesselID: u8,
    VesselName: String,
    Mmsi: Option<u32>,
    DepartingTerminalID: Option<u8>,
    DepartingTerminalName: Option<String>,
    DepartingTerminalAbbrev: Option<String>,
    ArrivingTerminalID: Option<u8>,
    ArrivingTerminalName: Option<String>,
    ArrivingTerminalAbbrev: Option<String>,
    pub Latitude: f64,
    pub Longitude: f64,
    pub Speed: f64,
    pub Heading: f64,
    InService: bool,
    AtDock: bool,
    LeftDock: Option<String>,
    Eta: Option<String>,
    EtaBasis: Option<String>,
    ScheduledDeparture: Option<String>,
    OpRouteAbbrev: Vec<String>,
    VesselPositionNum: Option<u8>,
    SortSeq: u16,
    ManagedBy: u8,
    pub TimeStamp: String,
    VesselWatchShutID: u8,
    VesselWatchShutMsg: String,
    VesselWatchShutFlag: String,
    VesselWatchStatus: String,
    VesselWatchMsg: String
}
