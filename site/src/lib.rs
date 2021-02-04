use actix_web::App;
use std::sync::{Arc, Mutex};

pub mod controller;
mod service;

pub struct Config {
    pub ip: String,
    pub port: u16,
}

pub struct AppState {
    connection_count: Mutex<u64>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            connection_count: Mutex::new(0),
        }
    }
}
