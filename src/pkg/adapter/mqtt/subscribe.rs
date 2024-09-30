
use paho_mqtt as mqtt;
use std::thread::sleep;
use crate::pkg::utils::{log, environment};

pub fn subscribe(cli: &mqtt::Client,topic: &str,conf:&environment::environment::Config) {
    if let Err(e) = cli.subscribe(topic, conf.broker.qos) {
        log::logger("error", "publish", &format!("failed subscribes topics [{:?}]: {:?}",topic, e));
        sleep(conf.broker.reconnect);
    }
}
