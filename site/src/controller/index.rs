use actix_web::{web, Responder, HttpResponse};
use crate::AppState;

pub async fn get_index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("I am up")
}