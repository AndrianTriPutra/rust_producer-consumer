
use std::{
    fs::File, 
    io::Read, 
};
use serde::{Serialize, Deserialize};
use crate::pkg::utils::domain;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: domain::config::General,  
    pub broker: domain::config::Broker,
    pub rabbit: domain::config::Rabbit,    
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_yaml::from_str(&contents)?;

        Ok(config)
    }
}

