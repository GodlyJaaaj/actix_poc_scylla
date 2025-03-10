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
use std::ops::{Deref, DerefMut};

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = schema::users)]
struct NewUser {
    name: String,
    email: String,
    phone: String
}

#[post("/register")]
async fn register(info: web::Json<NewUser>, db: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let mut conn = db.get().expect("Couldn't get DB connection from pool");

    let new_user = insert_into(users).values(&info.into_inner()).execute(&mut conn);

    match new_user {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", err));
        }
    }
    HttpResponse::Ok().body("Successfully registered")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = get_connection_pool();
    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api/auth")
                .app_data(web::Data::new(pool.clone()))
                .service(register))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
