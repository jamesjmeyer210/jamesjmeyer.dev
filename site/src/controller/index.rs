use actix_web::{web, Responder, HttpResponse};
use crate::AppState;

// TODO: read in the index.html file and its components and serve a response
pub async fn get_index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("I am up")
}