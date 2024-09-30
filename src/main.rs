use std::env;
use std::sync::Arc;

mod app;
mod pkg;

use crate::pkg::utils::environment::Config;
use crate::pkg::utils::log;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        log::logger("ftal", "main", "argument not found");
    }
    log::logger("info", "main", " ============================ [START] ============================");

    let mode = args[1..2].join(" ");
    let access = args[2..3].join(" ");
    let path = args[3..4].join(" ");
    log::logger("info", "main", &format!("mode  : {}", mode));
    log::logger("info", "main", &format!("access: {}", access));
    log::logger("info", "main", &format!("path  : {}", path));
    
    log::load(&access);
    
    let conf = Arc::new(Config::load(&path).expect("Failed to read config file"));
    log::logger("debug", "main", "============= CONFIG =============");
    log::logger("debug", "main", &format!("TZ       : {}", conf.general.tz));
    log::logger("debug", "main", &format!("PERIODIC : {:?}", conf.general.periodic));
    log::logger("debug", "main", "=============  MQTT  =============");
    log::logger("debug", "main", &format!("HOST     : {}", conf.broker.host));
    log::logger("debug", "main", &format!("USER     : {}", conf.broker.user));
    log::logger("debug", "main", &format!("PASS     : {}", conf.broker.pass));
    log::logger("debug", "main", &format!("QoS      : {}", conf.broker.qos));
    log::logger("debug", "main", &format!("TOPIC    : {}", conf.broker.topic));
    log::logger("debug", "main", &format!("RECON    : {:?}", conf.broker.reconnect));
    log::logger("debug", "main", &format!("RETRY    : {}", conf.broker.retries));
    log::logger("debug", "main", "============= RABBIT =============");
    log::logger("debug", "main", &format!("HOST     : {}", conf.rabbit.host));
    log::logger("debug", "main", &format!("TAG      : {}", conf.rabbit.tag));
    log::logger("debug", "main", &format!("QUE      : {}", conf.rabbit.que));
    log::logger("debug", "main", &format!("RECON    : {:?}", conf.rabbit.reconnect));
    log::logger("debug", "main", &format!("RETRY    : {}", conf.rabbit.retries));
    
    log::logger("info", "main", " ============================ [RUN] ============================");

    if mode=="producer"{
        app::producer::producer(conf).await;
    }else if mode=="consumer"{
        app::consumer::consumer(conf).await
    }else{
        log::logger("fatal", "main", &format!("mode Not Found ... !!!"));
    }

}


