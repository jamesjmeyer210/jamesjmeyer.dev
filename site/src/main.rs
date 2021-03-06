use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, HttpResponse};
use env_logger::Env;
use site::{controller, AppState, Config};
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config {
        ip: "127.0.0.1".to_string(),
        port: 8080,
    };

    let app_state = web::Data::new(AppState::new());
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(|data: web::Data<AppState>|{
                let content = data.web_pack.index.to_string();
                HttpResponse::Ok().body(content)
            }))
            .route("/resume", web::get().to(controller::get_resume))
            .route("/contact", web::get().to(controller::get_contact))
            .route("/blogs", web::get().to(controller::get_blogs))
            .route("/blog/{blog}", web::get().to(controller::get_blog))
            .route("/resource/{filename:.*}"
                   ,web::get().to(controller::get_file))
    })
    .bind(format!("{0}:{1}", config.ip, config.port))?
    .run()
    .await
}
