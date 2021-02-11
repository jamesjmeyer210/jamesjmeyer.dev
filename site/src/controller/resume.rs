use crate::AppState;
use std::fs;
use std::path::PathBuf;
use actix_web::{web, HttpResponse, Responder};
use actix_web::web::service;
use actix_web::http::StatusCode;

// TODO: read in the resume components and serve a resume html page
pub async fn get_resume(data: web::Data<AppState>) -> impl Responder {
    let path: std::path::PathBuf = PathBuf::from("resource/html/resume.html");
    match fs::read_to_string(path) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
