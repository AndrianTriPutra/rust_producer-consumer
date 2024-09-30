use paho_mqtt as mqtt;
use std::{
    time::Duration,
    thread::sleep,
};
use crate::pkg::utils::{log, environment};

pub fn connecting(client_id: &str, conf:&environment::environment::Config) -> (mqtt::Client, mqtt::Receiver<Option<mqtt::Message>>) {
    let mut attempts = 1;
    loop {
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(&conf.broker.host)
            .client_id(client_id)
            .finalize();

        let cli = match mqtt::Client::new(create_opts) {
            Ok(client) => client,
            Err(err) => {
                log::logger("error", "connection", &format!("failed creating the client: {:?}", err));
                sleep(conf.broker.reconnect);
                attempts += 1;
                if attempts > conf.broker.retries {
                    log::logger("fatal", "connection", &format!("[1] Exceeded maximum connection retries. Exiting...{:?}", attempts));
                }
                continue;
            }
        };

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .user_name(&conf.broker.user)
            .password(&conf.broker.pass)
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .finalize();

        if cli.connect(conn_opts).is_ok() {
            log::logger("info", "connection", &format!("Successfully connected after trying {:?} times", attempts));
            let rx = cli.start_consuming();
            return (cli, rx); // Return both cli and rx as a tuple
        } else {
            log::logger("error", "connection", &format!("Unable to connect. Retrying... attempts {:?}", attempts));
            sleep(conf.broker.reconnect);
            attempts += 1;
            if attempts > conf.broker.retries {
                log::logger("fatal", "connection", &format!("[2] Exceeded maximum connection retries. Exiting... {:?}", attempts));
            }
        }
    }
}

