pub mod config;
pub mod db;
pub mod models;
pub mod modules;
pub mod routes;
pub mod schema;
pub mod utils;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::{Logger, NormalizePath, TrailingSlash},
    web, App, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use env_logger::Env;
// use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::config::Config;
use crate::db::create_connection_pool;
use crate::routes::config_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration
    let config = Config::load();

    let port = config.port;

    // Set up database connection pool
    let db_pool = create_connection_pool(&config.database_url);

    // Set up redis store for sessions
    let redis_store = RedisSessionStore::new(config.redis_url.clone())
        .await
        .expect("Failed to connect to Redis");

    // Secret key for session
    let secret_key = Key::from(config.session_secret.as_bytes());

    log::info!("Starting server at http://0.0.0.0:{}", config.port);

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Authorization", "Content-Type", "Accept"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            // Enable logger
            .wrap(Logger::default())
            // Enable CORS
            .wrap(cors)
            // Normalize paths
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            // Identity middleware
            .wrap(IdentityMiddleware::default())
            // Session middleware
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_name("session_id".to_string())
                    .cookie_secure(config.env == "prod") // Only use secure cookies in production
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Lax)
                    .build(),
            )
            // Share database pool
            .app_data(web::Data::new(db_pool.clone()))
            // Share config
            .app_data(web::Data::new(config.clone()))
            // Swagger
            // .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls())
            // Configure API routes
            .configure(config_routes)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
