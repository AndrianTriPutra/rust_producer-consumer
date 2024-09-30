use paho_mqtt as paho;
use crate::pkg::adapter::mqtt;

use tokio::sync::mpsc;
use std::sync::Arc;
use crate::pkg::utils::environment::Config;
use crate::pkg::utils::{log, gettime};

pub async fn subscriber(conf: Arc<Config>, tx: mpsc::Sender<String>) {
    let mut failed: i8 = 0;
    let ct = gettime::current_time(conf.general.tz);
    let unix = ct.timestamp_micros().to_string();
    let client_id = format!("{}/{}", conf.broker.topic, unix);

    let (mut cli, mut rx) = mqtt::connect::connecting(&client_id, &conf); 
    mqtt::subscribe::subscribe(&cli, &conf.broker.topic, &conf);
    log::logger("info", "subscriber", "Ready to Receive Message . . .");

    // Call handler function to process messages and handle disconnections
    handler(&mut cli, &mut rx, &mut failed, conf.clone(), tx).await; // Pass conf as Arc

    if cli.is_connected() {
        log::logger("warning", "subscriber", "Disconnecting");
        cli.unsubscribe(&conf.broker.topic).unwrap();
        cli.disconnect(None).unwrap();
    }
    log::logger("warning", "subscriber", "Exiting");
}

// New handler function to process messages and handle disconnections
async fn handler(
    cli: &mut paho::Client,
    rx: &mut paho::Receiver<Option<paho::Message>>,
    failed: &mut i8,
    conf: Arc<Config>,
    tx: mpsc::Sender<String>,
) {
    loop {
        match rx.recv() { 
            Ok(Some(msg)) => {
                log::logger("trace", "handler", &format!("Received topic {:?}", msg.topic()));
                if msg.topic().contains("data") {
                    log::logger("trace", "handler", &format!("Received msg {:?}", msg.payload_str()));
                    if let Err(e) = tx.send(msg.payload_str().to_string()).await {
                        log::logger("error", "handler", &format!("Failed to send payload {} : {:?}", msg.payload_str(), e));
                    }
                }
            }
            Ok(None) | Err(_) => {
                if !cli.is_connected() {
                    *failed += 1;
                    log::logger("error", "handler", &format!("Client disconnected. Attempting to reconnect... Retry [{:?}]", *failed));
                    if *failed > conf.broker.retries {
                        log::logger("fatal", "handler", "Exceeded maximum retries. Exiting");
                    }

                    let ct = gettime::current_time(conf.general.tz);
                    let unix = ct.timestamp_micros().to_string();
                    let client_id = format!("{}/{}", conf.broker.topic, unix);

                    // Reconnect and reassign `cli` and `rx`
                    let (new_cli, new_rx) = mqtt::connect::connecting(&client_id, &conf); 
                    *cli = new_cli;
                    *rx = new_rx;

                    log::logger("info", "handler", "Resubscribe topics...");
                    mqtt::subscribe::subscribe(cli, &conf.broker.topic, &conf); 
                }
            }
        }
    }
}
