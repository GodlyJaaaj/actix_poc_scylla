use crate::models::User;
use crate::schema::users::dsl::users;
use crate::schema::users::email;
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::{DatabaseErrorKind, Error};
use diesel::{
    insert_into, ExpressionMethods, Insertable, PgConnection, RunQueryDsl, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Insertable, ToSchema, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewUserQuery {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, ToSchema, Serialize)]
struct NewUserResponse {
    uuid: String,
    name: String,
    email: String,
    created_at: String,
}

#[utoipa::path(
    context_path = "/api/auth",
    request_body = NewUserQuery,
    responses(
            (status = 201, description = "User created successfully", body = NewUserResponse),
            (status = 409, description = "User already exists"),
    )
)]
#[post("/register")]
async fn register(
    mut info: web::Json<NewUserQuery>,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = db
        .get()
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .body(format!("Diesel error occurred getting connection: {}", e))
        })
        .unwrap();

    let hashed_password = hash(&info.password, DEFAULT_COST)
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
        .unwrap();

    info.password = hashed_password.to_string();

    let inserted_user = insert_into(users)
        .values(&info.into_inner())
        .returning(User::as_returning())
        .get_result(&mut conn);

    match inserted_user {
        Ok(user) => {
            let response = NewUserResponse {
                uuid: user.id.to_string(),
                name: user.name,
                email: user.email,
                created_at: user
                    .created_at
                    .unwrap_or(NaiveDateTime::default())
                    .to_string(),
            };
            HttpResponse::Created().json(response)
        }
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            HttpResponse::Conflict().body("User already with this email already exists")
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error while creating user: {:?}", e))
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
struct LoginQuery {
    email: String,
    password: String,
}

#[utoipa::path(
    context_path = "/api/auth",
    request_body = LoginQuery,
    responses(
            (status = 201, description = "Logged in successfully", body = String),
    )
)]
#[post("/login")]
async fn login(
    login_query: web::Json<LoginQuery>,
    session: Session,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = db
        .get()
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .body(format!("Diesel error occurred getting connection: {}", e))
        })
        .unwrap();

    let user = users
        .filter(email.eq(&login_query.email))
        .first::<User>(&mut conn);

    if let Err(_) = user {
        return HttpResponse::Unauthorized().body("Invalid email");
    }

    let user = user.unwrap();

    if let Err(_) = verify(&login_query.password, &user.password) {
        println!("{} {}", &user.password, login_query.password);
        return HttpResponse::Unauthorized().body("Invalid password");
    }

    session.insert("user_id", user.id.to_string()).unwrap();

    HttpResponse::Ok().body("Logged in")
}
