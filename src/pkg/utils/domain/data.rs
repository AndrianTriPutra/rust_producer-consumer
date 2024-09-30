use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Sensor {
    pub sensor_id: u32,
    pub temp: f64,
    pub rh: f64,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub dev_id: String,
    pub ts: String,
    pub data: Vec<Sensor>,
}

pub fn decode(data: String) -> Result<Data, serde_json::Error> {
    let device_data: Data = serde_json::from_str(&data)?;
    Ok(device_data)
}