use actix_web::App;
use std::sync::{Arc, Mutex};

pub mod controller;

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

    // pub fn get_conn(&mut self) -> u64 {
    //     self.connection_count
    //         .into_inner()
    //         .unwrap()
    //         .clone()
    // }
    //
    // pub fn inc_conn(&mut self) -> () {
    //     let conn = self.connection_count
    //         .get_mut()
    //         .unwrap();
    //     *conn += 1;
    // }
}
