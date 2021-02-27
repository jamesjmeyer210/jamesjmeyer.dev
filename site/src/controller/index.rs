use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{web, HttpResponse, Responder};
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use crate::AppState;

pub async fn get_index(data: web::Data<AppState>) -> impl Responder {
    match fs::read_to_string("resource/index.html") {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}