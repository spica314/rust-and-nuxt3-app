use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    app_config::*,
    state::AppState,
    v1::{add_int::v1_add_int, get_int::v1_get_int},
};

pub async fn run_app(config: AppConfig) {
    let state = AppState {
        value: Arc::new(Mutex::new(config.initial_value)),
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        // Expanded from the default size (256MB)
        // (This is not necessary here, but we have configured it because it is often required for our other purposes.)
        let json_cfg = web::JsonConfig::default().limit(256 * 1048576);

        App::new()
            .wrap(cors)
            .app_data(json_cfg)
            .app_data(web::Data::new(state.clone()))
            .service(v1_add_int)
            .service(v1_get_int)
    })
    .bind(("0.0.0.0", config.port))
    .unwrap()
    .run()
    .await
    .unwrap();
}
