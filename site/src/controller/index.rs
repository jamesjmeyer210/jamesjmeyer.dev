use actix_web::{web, Responder, HttpResponse};
use crate::AppState;
use std::rc::Rc;

// TODO: read in the index.html file and its components and serve a response
pub async fn get_index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("I am up")
}

struct IndexView {
    template: Option<&'static str>,
    nav_bar: Option<&'static str>,
    resume: Option<Rc<str>>,
    blogs: Option<Rc<str>>,
}

impl IndexView {

    fn new() -> Self {
        IndexView {
            template: None,
            nav_bar: None,
            resume: None,
            blogs: None,
        }
    }

    fn set_template(&mut self, html: &'static str) -> &mut Self {
        self.template = Some(html);
        self
    }

    fn set_navbar(&mut self, html: &'static str) -> &mut Self {
        self.nav_bar = Some(html);
        self
    }

    // TODO: retrieve resume results from resume service
    fn set_resume(&mut self, html: Rc<str>) -> &mut Self {
        self.resume = Some(html);
        self
    }

    // TODO: retrieve blog urls from blog service
    fn set_blogs(&mut self, html: Rc<str>) -> &mut Self {
        self.blogs = Some(html);
        self
    }
}