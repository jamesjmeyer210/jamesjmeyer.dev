use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{web, HttpResponse, Responder, HttpRequest, Error};
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use crate::AppState;
use comrak::{markdown_to_html, ComrakOptions};

pub async fn get_index(data: web::Data<AppState>) -> impl Responder {
    let content = data.web_pack.index.to_string();
    HttpResponse::Ok().body(content)
}

pub async fn get_resume(data: web::Data<AppState>) -> impl Responder {
    let content = data.web_pack.resume.to_string();
    HttpResponse::Ok().body(content)
}

pub async fn get_contact(data: web::Data<AppState>) -> impl Responder {
    let content = data.web_pack.contact.to_string();
    HttpResponse::Ok().body(content)
}

pub async fn get_blogs(data: web::Data<AppState>) -> impl Responder
{
    let mut content = String::new();
    data.web_pack.blogs.iter().for_each(|kvp|{
        let title = kvp.0.replace('_', " ");

        let sub_path = kvp.0; //.trim_end_matches(".md");

        content.push_str(&format!("[{0}](/blog/{1})\n\n"
                                  , title
                                  , sub_path));
    });
    let html = markdown_to_html(&content, &ComrakOptions::default());
    HttpResponse::Ok().body(html)
}

pub async fn get_blog(data: web::Data<AppState>, blog: web::Path<String>) -> impl Responder
{
    println!("Looking up blog: {0}", blog.as_str());

    match data.web_pack.blogs.contains_key(blog.as_str())
    {
        true => {
            let md = data.web_pack.blogs
                .get(blog.as_str())
                .unwrap()
                .as_str();
            let html = markdown_to_html(md, &ComrakOptions::default());

            HttpResponse::Ok().body(html)
        }
        false => HttpResponse::NotFound().finish()
    }
}

pub async fn get_file(req: HttpRequest) -> Result<actix_files::NamedFile, Error>{
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = actix_files::NamedFile::open(format!("resource/{0}", path.to_str().unwrap()))?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}