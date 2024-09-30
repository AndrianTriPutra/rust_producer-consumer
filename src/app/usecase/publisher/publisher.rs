use std::sync::Arc;
use crate::pkg::utils::environment::Config;

use tokio::sync::mpsc;
use crate::pkg::utils::log;
use crate::pkg::utils::domain;
use crate::pkg::adapter::rabbit;

pub async fn publisher(mut rx: mpsc::Receiver<String>, conf: Arc<Config>) {
    let mut failed=0;
    let mut channel = None; 
    
    loop {
        if channel.is_none() {
            match rabbit::connect::connection(&conf).await {
                Ok(new_channel) => {
                    failed=0;
                    channel = Some(new_channel);
                    log::logger("info", "publisher", "Successfully connected to RabbitMQ");
                }
                Err(err) => {
                    failed+=1;
                    log::logger("error", "publisher", &format!("Failed to connect was [{}] and retry reconnect, error: {}",failed ,err));
                    if failed>conf.rabbit.retries{
                        log::logger("fatal", "publisher", "failed rabbit more than retries");
                    }
                    // Wait before retrying
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; 
                }
            }
        }

        match rx.recv().await {
            Some(data) => {
                log::logger("info", "publisher", &format!("data : {}", data));
                match domain::data::decode(data.to_string()) {
                    Ok(payload) => {
                        log::logger("debug", "publisher", &format!("device_id      : {}", payload.dev_id));
                        log::logger("debug", "publisher", &format!("timestamp      : {}", payload.ts));
                        log::logger("debug", "publisher", &format!(" +++++++++++++++++ [SENSOR] +++++++++++++++++ "));
                        for sensor in payload.data {
                            log::logger("debug", "publisher", &format!("sensor_id  : {}", sensor.sensor_id));
                            log::logger("debug", "publisher", &format!("temperatur : {}", sensor.temp));
                            log::logger("debug", "publisher", &format!("humudity   : {}", sensor.rh));
                            log::logger("debug", "publisher", &format!(" ++++++++++++++++++++++++++++++++++"));
                        }
                        
                        if let Some(ref ch) = channel {
                            // Publish a message
                            match rabbit::publish::publish(ch, &conf.rabbit.que, &data).await {
                                Ok(_) => {
                                    log::logger("info", "publisher", "Message published successfully.");
                                }
                                Err(err) => {
                                    log::logger("error", "publisher", &format!("Failed to publish message: {}", err));
                                    channel = None; 
                                }
                            }                                   
                        }
                    }
                    
                    Err(e) => {
                        log::logger("error", "publisher", &format!("Failed to decode JSON: {:?}", e));
                    }
                }
            }
            None => {
                log::logger("warning", "publisher", &format!("Channel closed"));
                return; 
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; 
    }
}