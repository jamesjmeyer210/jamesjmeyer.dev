use actix_web::{web, App, HttpServer, middleware::Logger, HttpRequest};
use site::{Config, AppState, controller};
use std::sync::Mutex;
use env_logger::Env;

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
            .route("/", web::get().to(controller::get_index))
            .route("/resume", web::get().to(controller::get_resume))
            .route("/css/{filename:.*}", web::get().to(controller::get_css))
            .route("/js/{filename:.*}", web::get().to(controller::get_js))
    })
    .bind(format!("{0}:{1}", config.ip, config.port))?
    .run()
    .await
}
