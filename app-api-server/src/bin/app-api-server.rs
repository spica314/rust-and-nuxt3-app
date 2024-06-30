use app_api_server::app::run_app;
use app_api_server::app_config::AppConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    let initial_value = std::env::var("INITIAL_VALUE")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .expect("INITIAL_VALUE must be a number");

    let config = AppConfig {
        initial_value,
        port,
    };

    run_app(config).await;
}
