use actix_web::{web, Responder, HttpResponse};
use crate::AppState;

// TODO: read in the resume components and serve a resume html page
pub async fn get_resume(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("This is the resume page")
}