use crate::AppState;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{web, HttpResponse, Responder};
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

pub async fn get_index(data: web::Data<AppState>) -> impl Responder {
    let path: std::path::PathBuf = PathBuf::from("resource/index.html");
    match fs::read_to_string(path) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
