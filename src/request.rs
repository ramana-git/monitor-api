use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug,Deserialize,Serialize)]
pub struct HealthRequest {
    pub uuid: Uuid,
    pub app_name: String,
    pub api_name: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub interval: i32, //seconds default 120 min
    pub timeout: i32,  //seconds default 5 seconds
}
impl Default for HealthRequest {
    fn default() -> Self {
        HealthRequest {
            uuid: Uuid::nil(),
            app_name: String::new(),
            api_name: String::new(),
            url: String::new(),
            headers: HashMap::new(),
            interval: 120,
            timeout: 5
        }
    }
}
impl HealthRequest {}

#[derive(Debug,Deserialize,Serialize)]
pub struct HealthHistory {
    pub uuid: Uuid,
    pub time: i64,
    pub duration: i32, //milliseconds
    pub health: bool,
    pub code: i16,
}