use std::{fs, io};
use actix_web::Error;
use actix_web::http::header::{ContentDisposition, DispositionType};

pub fn get_resource(path: &str) -> io::Result<String>  {
    fs::read_to_string(path)
}