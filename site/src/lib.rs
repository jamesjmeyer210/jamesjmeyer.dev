use actix_web::App;
use std::sync::{Arc, Mutex};
use crate::resource::WebPack;

//pub mod view;
mod resource;
pub mod controller;

pub struct Config {
    pub ip: String,
    pub port: u16,
}

pub struct AppState {
    connection_count: Mutex<u64>,
    pub web_pack: Arc<WebPack>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            connection_count: Mutex::new(0),
            web_pack: Arc::new(WebPack::new()),
        }
    }
}
