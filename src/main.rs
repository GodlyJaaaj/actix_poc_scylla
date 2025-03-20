pub mod config;
pub mod db;
pub mod models;
pub mod modules;
pub mod routes;
pub mod schema;
pub mod utils;

use crate::config::Config;
use crate::db::create_connection_pool;
use crate::routes::config_routes;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::{Logger, NormalizePath, TrailingSlash},
    web, App, HttpResponse, HttpServer,
};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration
    let config = Config::load();

    // Set up database connection pool
    let db_pool = create_connection_pool(&config.database.url);

    // Set up redis store for sessions
    let redis_store = RedisSessionStore::new(&config.redis.url)
        .await
        .expect("Failed to connect to Redis");

    // Secret key for session
    let secret_key = Key::from(config.session.secret.as_bytes());

    let server_addr = format!("{}:{}", config.server.base_url, config.server.port);
    log::info!(
        "Starting server at {}://{}",
        config.server.protocol,
        server_addr
    );

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin(&format!(
                "{}://{}:{}",
                config.server.protocol, config.server.base_url, config.server.port
            ))
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Authorization", "Content-Type", "Accept"])
            .supports_credentials()
            .max_age(3600);

        // Create and configure app
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_name("session_id".to_string())
                    .cookie_secure(config.server.env == "prod")
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Lax)
                    .build(),
            )
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(config_routes)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind(&server_addr)?
    .run()
    .await
}
