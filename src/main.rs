pub mod api;
pub mod models;
pub mod schema;

use crate::api::UserApi;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use env_logger::Env;
use std::env;
use std::net::SocketAddrV4;
use actix_identity::IdentityMiddleware;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let socket = SocketAddrV4::new("127.0.0.1".parse().unwrap(), 8080);

    eprintln!("Listening on : http://{:?}", socket);
    let storage = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    let key = Key::generate();

    let pool = get_connection_pool();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(storage.clone(), key.clone()))
            .service(
                web::scope("/api/auth")
                    .app_data(web::Data::new(pool.clone()))
                    .service(crate::api::user::register)
                    .service(crate::api::user::login)
                    .service(crate::api::user::logout),
            )
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
                Url::new("user-api", "/api-docs/user-api.json"),
                UserApi::openapi(),
            )]))
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind(socket)?
    .run()
    .await
}
