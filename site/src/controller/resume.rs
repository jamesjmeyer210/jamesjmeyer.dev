use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

// TODO: read in the resume components and serve a resume html page
pub async fn get_resume(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .set_header("Location", "/resource/html/resume.html")
        .finish()
    //HttpResponse::Ok().body("This is the resume page")
}
