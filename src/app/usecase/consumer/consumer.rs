use std::sync::Arc;
use crate::pkg::utils::environment::Config;
use crate::pkg::adapter::rabbit;
use crate::pkg::utils::log;

pub async fn consumers(conf: Arc<Config>){
    let mut failed=0;
    // Loop to handle reconnection attempts
    loop {
        match rabbit::connect::connection(&conf).await {
            Ok(channel) => {
                failed=0;
                log::logger("info", "connection", "Successfully connected to RabbitMQ");
                    
                // Start consuming messages
                if let Ok(consumer) = rabbit::consume::consume(&channel, &conf.rabbit.tag, &conf.rabbit.que).await {
                    // Handle messages
                    rabbit::consume::handler(consumer).await;
                }
            }
            Err(err) => {
                failed+=1;
                log::logger("error", "connection", &format!("Failed to connect was [{}] and retry reconnect, error: {}",failed ,err));
                if failed>conf.rabbit.retries{
                    log::logger("fatal", "connection", "failed rabbit more than retries");
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await; 
            }
        }
    }
}