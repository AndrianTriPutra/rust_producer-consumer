use futures::StreamExt;
use lapin::{options::*, types::FieldTable, Channel, Consumer, message::Delivery};
use crate::pkg::utils::log;

pub async fn consume(channel: &Channel, tag: &str, que: &str) -> Result<Consumer, lapin::Error> {
    let consumer = channel
        .basic_consume(
            que,
            tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(consumer)
}

pub async fn handler(mut consumer: Consumer) {
    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => received(delivery).await,
            Err(err) => log::logger("error", "handler", &format!("Error in consumer: {}", err)),
        }
    }
}

async fn received(delivery: Delivery) {
    log::logger("debug", "received", &format!("Received message: {}", String::from_utf8_lossy(&delivery.data)));

    // Acknowledge the message
    if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
        log::logger("error", "received", &format!("Failed to ack: {}", err));
    }
}
