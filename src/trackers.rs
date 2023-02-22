use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use std::error::Error;
use tiberius::time::chrono::NaiveDateTime;
use uuid::Uuid;

use crate::request::{HealthHistory,HealthRequest};

pub async fn connect(conn_str: &str,max_size: u8) -> Result<Pool<ConnectionManager>, Box<dyn Error>> {
    let mgr = ConnectionManager::build(conn_str)?;
    let pool = Pool::builder().max_size(max_size.into()).build(mgr).await?;
    Ok(pool)
}

pub async fn requests(pool: &Pool<ConnectionManager>, schema: &str) -> Result<Vec<HealthRequest>, Box<dyn Error>> {
    println!("Getting connection...from pool");
    let mut conn = pool.get().await.unwrap();
    println!("Got connection");
    let result: Vec<HealthRequest> = conn
        .simple_query(format!("select * from {schema}.HealthTrackers where active=1"))
        .await?
        .into_first_result()
        .await?
        .into_iter()
        .map(|row| -> HealthRequest {
            let headers = row.get::<&str, &str>("headers").unwrap();
            HealthRequest {
                uuid: row.get::<Uuid, &str>("tid").unwrap(),
                app_name: row.get::<&str, &str>("appname").unwrap().to_owned(),
                api_name: row.get::<&str, &str>("apiname").unwrap().to_owned(),
                url: row.get::<&str, &str>("url").unwrap().to_owned(),
                headers: serde_json::from_str(headers).unwrap_or_default(),
                interval: row.get::<i32, &str>("interval").unwrap_or_default(),
                timeout: row.get::<i32, &str>("timeout").unwrap_or_default(),
            }
        })
        .collect();
    Ok(result)
}

pub async fn history(pool: &Pool<ConnectionManager>, schema: &str, tid: &Uuid, count: u8) -> Result<Vec<HealthHistory>, Box<dyn Error>> {
    println!("Getting connection...from pool");
    let mut conn = pool.get().await.unwrap();
    println!("Got connection");
    let result: Vec<HealthHistory> = conn
        .simple_query(format!("select top {count} * from {schema}.HealthHistory where tid='{tid}'"))
        .await?
        .into_first_result()
        .await?
        .into_iter()
        .map(|row| -> HealthHistory {
            HealthHistory {
                uuid: row.get::<Uuid, &str>("tid").unwrap(),
                time: row.get::<NaiveDateTime, &str>("checktime").unwrap().timestamp_millis(),
                duration: row.get::<i32, &str>("duration").unwrap(),
                health: row.get::<bool, &str>("health").unwrap(),
                code: row.get::<i16, &str>("code").unwrap()
            }
        })
        .collect();
    Ok(result)
}
