/*
use paho_mqtt as mqtt;
use std::thread::sleep;

use crate::pkg::utils::{log, environment};

pub fn publish(cli: &mqtt::Client, topic: &str, content: &str, conf:&environment::environment::Config) {
    let msg = mqtt::Message::new(topic.to_string(), content.to_string(), conf.broker.qos);
    log::logger("info", "publish", &format!("publish to {:?} with msg {:?}", msg.topic(), msg.payload_str()));

    if let Err(e) = cli.publish(msg) {
        log::logger("error", "publish", &format!("failed sending message: {:?}", e));
        // Handle the publish error by waiting before retrying
        sleep(conf.broker.reconnect);
    }
}
*/
