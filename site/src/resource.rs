use std::fs;
use std::collections::HashMap;

pub struct WebPack
{
    pub index: Box<String>,
    pub resume: Box<String>,
    pub contact: Box<String>,
    pub blogs: HashMap<String,Box<String>>,
    pub css: HashMap<String,Box<String>>,
}

impl WebPack
{
    pub fn new() -> Self
    {
        WebPack
        {
            index: load_file("resource/index.html"),
            resume: load_file("resource/resume.html"),
            contact: load_file("resource/contact.html"),
            blogs: load_dir("resource/blog"),
            css: load_dir("resource/css"),
        }
    }
}

fn load_file(path: &'static str) -> Box<String>
{
    println!("Attempting to load {0}", path);
    Box::new(fs::read_to_string(path).unwrap())
}

fn load_dir(path: &'static str) -> HashMap<String,Box<String>>
{
    println!("Attempting to load {0}", path);
    fs::read_dir(path)
        .unwrap()
        .map(|file|{
            let f = file.unwrap();
            let f_name = f.file_name().into_string().unwrap();
            let f_content = fs::read_to_string(f.path()).unwrap();
            // TODO: use the logger
            println!("Loaded: {0}", &f_name);
            (f_name,Box::new(f_content))
        })
        .collect()
}