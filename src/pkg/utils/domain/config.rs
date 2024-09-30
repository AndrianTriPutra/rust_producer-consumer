
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub tz: bool,
    #[serde(with = "humantime_serde")] 
    pub periodic: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rabbit {
    pub host: String,     
    pub tag : String,
    pub que : String,
    #[serde(with = "humantime_serde")] 
    pub reconnect: Duration,
    pub retries: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Broker {
    pub host: String,
    pub user: String,
    pub pass: String,
    pub qos: i32,
    pub topic: String,
    #[serde(with = "humantime_serde")] 
    pub reconnect: Duration,
    pub retries: i8,
}
