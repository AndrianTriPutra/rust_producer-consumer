use tokio::sync::mpsc;
use std::sync::Arc;

use crate::{app, pkg};

pub async fn producer(conf: Arc<pkg::utils::environment::Config>){
    let subs_conf = Arc::clone(&conf);
    let pubs_conf = Arc::clone(&conf);

    let (tx, rx) = mpsc::channel::<String>(1); 
    //let (tx, rx) = mpsc::channel::<String>(32); //if You expect a higher flow of data and want to avoid blocking
    let duration = tokio::time::Duration::from_secs(conf.general.periodic.as_secs());

    let subs_handle = tokio::spawn(app::usecase::publisher::subscriber::subscriber(subs_conf, tx));
    let pub_handle = tokio::spawn(app::usecase::publisher::publisher::publisher(rx,pubs_conf));
    let mem_handle = tokio::spawn(pkg::repository::memory::memory::memory_check(duration));

    let _ = tokio::join!(subs_handle, pub_handle, mem_handle);
}