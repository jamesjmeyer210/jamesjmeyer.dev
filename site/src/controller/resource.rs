use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, App, Error, HttpRequest, HttpServer};
use log::{info};

pub async fn get_css(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    info!("Path identified: {}", path.to_str().unwrap());
    let file = fs::NamedFile::open(format!("static/css/{0}", path.to_str().unwrap()))?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

pub async fn get_js(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    info!("Path identified: {}", path.to_str().unwrap());
    let file = fs::NamedFile::open(format!("static/js/{0}", path.to_str().unwrap()))?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}