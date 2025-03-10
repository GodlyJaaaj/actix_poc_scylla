pub mod models;
pub mod schema;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use schema::users::dsl::*;
use serde::Deserialize;
use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};
use actix_web::middleware::Logger;
use diesel::result::Error;
use env_logger::Env;
use log::info;
use crate::models::User;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}

#[derive(Deserialize, Insertable, Debug, Queryable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewUser {
    name: String,
    email: String
}

#[post("/register")]
async fn register(info: web::Json<NewUser>, db: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let mut conn = db.get().expect("Couldn't get DB connection from pool");

    // check if already exists
    let check = users.filter(email.eq(&info.email)).select(User::as_select()).first(&mut conn);
    match check {
        Ok(_user) => {
            HttpResponse::BadRequest().json("User with this email already exists")
        }
        Err(Error::NotFound) => {
            let inserted_user =
                insert_into(users).values(&info.into_inner()).execute(&mut conn);

            match inserted_user {
                Ok(user) => HttpResponse::Created().json(user),
                Err(_) => HttpResponse::InternalServerError().json("Error creating user"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Database error"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);

    eprintln!("Listening on : http://{:?}", socket);

    let pool = get_connection_pool();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api/auth")
                .app_data(web::Data::new(pool.clone()))
                .service(register))
    })
    .bind(socket)?
    .run()
    .await
}
