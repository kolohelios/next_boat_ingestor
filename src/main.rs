use actix_web::{get, web, App, HttpServer, HttpResponse, client::Client};
use deadpool_redis::{cmd, Pool};
use dotenv::dotenv;
use chrono::prelude::*;

mod config;
mod models;
mod utils;

#[get("/")]
async fn index(pool: web::Data<Pool>) -> Result<HttpResponse, models::Error> {
    let client = Client::default();

    let cfg = config::Config::from_env().unwrap();
    let api_key = cfg.wsf_api_key;

    // let dt = Local::now();

    // let response = client.get(format!("https://www.wsdot.wa.gov/ferries/api/schedule/rest/terminals/{}-{}-{}?apiaccesscode={}", dt.year(), dt.month(), dt.day(), api_key))
    //     .header("User-Agent", "actix-web/3.0")
    //     .send()
    //     .await
    //     .unwrap()
    //     .json::<Vec<models::Terminal>>()
    //     .await;

    let mut conn = pool.get().await?;


    let response = client.get(format!("https://www.wsdot.wa.gov/ferries/api/vessels/rest/vessellocations?apiaccesscode={}", api_key))
        .header("User-Agent", "actix-web/3.0")
        .send()
        .await
        .unwrap()
        .json::<Vec<models::VesselLocation>>()
        .await;

    // TODO: fix this so we don't have the nested for loop to get the data out of the response
    for data in response {
        for vessel_location in data {
            let time_stamp = utils::time_stamp_converter(&vessel_location.TimeStamp);

            let dt = Utc.timestamp(time_stamp, 0);

            let data_points = vec![
                ("latitude", vessel_location.Latitude),
                ("longitude", vessel_location.Longitude),
                ("speed", vessel_location.Speed),
                ("heading", vessel_location.Heading),
            ];

            let vessel_id = &vessel_location.VesselID;

            for data in data_points {
                let key = format!("v:{}:{}", vessel_id, data.0);
                cmd("TS.CREATE").arg(&[&key]).execute_async(&mut conn).await.ok();
                let _ = cmd("TS.ADD").arg(&key).arg(dt.timestamp()).arg(data.1).execute_async(&mut conn).await;
            }
        }
    }

    // let key = String::from("test2");
    // let value = 2848490;
    // let now = Utc::now().timestamp();

    // Swallow errors if the time series already exists
    // cmd("TS.CREATE").arg(&[&key]).execute_async(&mut conn).await.ok();
    // let _ = cmd("TS.ADD").arg(&key).arg(now).arg(value).execute_async(&mut conn).await;
    // let retrieved_value: (u64, String) = cmd("TS.GET").arg(&[&key]).query_async(&mut conn).await.unwrap();
    // let new_value = retrieved_value.1;
    Ok(HttpResponse::Ok().body(format!("OK")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let cfg = config::Config::from_env().unwrap();
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
