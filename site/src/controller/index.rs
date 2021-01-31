use actix_web::{web, Responder, HttpResponse};
use crate::AppState;
use std::fs;
use std::rc::Rc;
use std::path::PathBuf;
use actix_web::http::header::{ContentDisposition, DispositionType};

pub async fn get_index(data: web::Data<AppState>) -> impl Responder {
    let path: std::path::PathBuf = PathBuf::from("static/index.html");
    match fs::read_to_string(path) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}