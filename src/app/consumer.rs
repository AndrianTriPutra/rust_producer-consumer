use std::sync::Arc;
use crate::{app, pkg};

pub async fn consumer(conf: Arc<pkg::utils::environment::Config>){
    app::usecase::consumer::consumer::consumers(conf).await
}