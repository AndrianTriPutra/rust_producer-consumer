use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties};

use crate::pkg::utils::{log, environment};

pub async fn connection(conf:&environment::environment::Config) -> Result<Channel, lapin::Error> {
    let conn = Connection::connect(&conf.rabbit.host, ConnectionProperties::default()).await?;

    // Create a channel
    let channel = conn.create_channel().await?;

    // Declare a queue
    let queue = channel
        .queue_declare(
            &conf.rabbit.que,
            QueueDeclareOptions {
                durable: true,
                auto_delete: false,
                exclusive: false,
                nowait: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    log::logger("info", "connection", &format!("Declared queue: {:?}", queue));

    Ok(channel)
}

