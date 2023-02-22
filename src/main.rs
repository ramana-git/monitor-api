use std::{env, net::SocketAddr};
use axum::{routing::get, Json, Router, Extension, extract::{Path, Query}};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use monitor_api::{trackers::{connect, requests, history}, request::{HealthRequest, HealthHistory}};
use tiberius::Uuid;
use tower_http::cors::{CorsLayer, Any};
use std::collections::HashMap;

#[derive(Clone)]
struct Connection{
    pool: Pool<ConnectionManager>,
    schema: String
}

#[tokio::main]
async fn main() {
    let conn=db_connect().await;
    let app = Router::new().route("/", get(root))
        .route("/trackers", get(get_trackers))
        .route("/history/:id", get(get_history));

    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8888);

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("Server Listening on {:?}",address);
    axum::Server::bind(&address)
        .serve(app.layer(CorsLayer::new().allow_origin(Any)).layer(Extension(conn)).into_make_service())
        .await
        .unwrap();    
}

async fn root() -> &'static str {
    "{{\"message\":\"Read documentation\"}}"
}

async fn db_connect() -> Connection {
    let server = match env::var("SERVER") {
        Ok(value) => value,
        Err(_e) => "localhost".to_owned(),
    };
    let port = match env::var("PORT") {
        Ok(value) => value,
        Err(_e) => "1433".to_owned(),
    };
    let database = match env::var("DATABASE") {
        Ok(value) => value,
        Err(_e) => "master".to_owned(),
    };
    let schema = match env::var("DB_SCHEMA") {
        Ok(value) => value,
        Err(_e) => "dbo".to_owned(),
    };
    let user = match env::var("DB_USER") {
        Ok(value) => value,
        Err(_e) => "sa".to_owned(),
    };
    let password = match env::var("DB_PASSWORD") {
        Ok(value) => value,
        Err(_e) => "YourStrong!Passw0rd".to_owned(),
    };
    let max_pool_size = match env::var("DB_MAX_POOL_SIZE") {
        Ok(value) => value.parse::<u8>().unwrap(),
        Err(_e) => 3,
    };
    let conn_str = format!(
        "Server={server};Port={port};Database={database};User Id={user};Password={password};"
    );
    println!("Connection string: {}", conn_str);

    let pool = connect(&conn_str, max_pool_size).await.unwrap();
    Connection{pool,schema}
}

async fn get_trackers(Extension(conn): Extension<Connection>)->Json<Vec<HealthRequest>> {
    let trackers = requests(&conn.pool, &conn.schema).await.unwrap();
    println!("{:#?}", trackers);
    Json(trackers)
}

async fn get_history(Extension(conn): Extension<Connection>, Path(id): Path<Uuid>, Query(params): Query<HashMap<String, String>>)->Json<Vec<HealthHistory>>{    
    let count=params.get("count").unwrap_or(&"1".to_owned()).parse().unwrap_or(1_u8);
    let data=history(&conn.pool, &conn.schema, &id,count).await.unwrap();    
    println!("{:#?}", serde_json::to_string(&data).unwrap());
    Json(data)
}