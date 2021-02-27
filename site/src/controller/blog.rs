use actix_web::{web, Responder, HttpResponse};
use crate::AppState;
use std::fs;
use std::ffi::OsString;
use std::ops::Add;
use comrak::{markdown_to_html, ComrakOptions};

pub async fn get_blogs(data: web::Data<AppState>) -> impl Responder {
    let blogs: Vec<String> = fs::read_dir("resource/blogs")
        .unwrap()
        .map(|e|{
            e.unwrap()
                .file_name()
                .into_string()
                .unwrap()
        })
        .collect();

    match blogs.len()
    {
        0 => HttpResponse::NoContent().finish(),
        _ => {
            let markdown = render_blog_list(&blogs);
            let html = markdown_to_html(&markdown, &ComrakOptions::default());
            HttpResponse::Ok().body(html)
        }
    }
}

fn render_blog_list(blogs: &Vec<String>) -> String {
    let mut md = String::from("# Blogs\n\n");

    for blog in blogs.iter() {
        md.push_str(&format!("[{0}](blogs/{1})\n\n"
                             , blog.replace('_', " ").replace(".md", "")
                             , blog))
    }

    md
}