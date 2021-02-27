use actix_web::{Responder, HttpResponse};
use std::fs;

pub async fn get_contact() -> impl Responder {
    match fs::read_to_string("resource/html/contact.html") {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}