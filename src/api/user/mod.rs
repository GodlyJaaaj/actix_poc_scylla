use crate::models::User;
use crate::schema::users::dsl::users;
use crate::schema::users::{email, id, password};
use actix_identity::Identity;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
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
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Insertable, ToSchema, Debug, Validate)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct RegisterQuery {
    #[schema(example = "John Doe")]
    #[validate(length(min = 3, max = 100))]
    name: String,

    #[schema(example = "john.doe@gmail.com")]
    #[validate(email)]
    email: String,

    #[schema(example = "my super password")]
    #[validate(length(min = 8))]
    password: String,
}

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct RegisterResponse {
    id: Uuid,

    #[schema(example = "John Doe")]
    name: String,

    #[schema(example = "john.doe@gmail.com")]
    email: String,
    created_at: NaiveDateTime,
}

#[utoipa::path(
    context_path = "/api/auth",
    request_body = RegisterQuery,
    tags=["Auth"],
    responses(
            (status = 201, description = "User created successfully", body = RegisterResponse),
            (status = 400, description = "Validation error"),
            (status = 409, description = "User already exists"),
    )
)]
#[post("/register")]
async fn register(
    mut info: web::Json<RegisterQuery>,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    if let Err(e) = info.validate() {
        return HttpResponse::BadRequest().body(e.to_string());
    }

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
        .returning(RegisterResponse::as_returning())
        .get_result(&mut conn);

    match inserted_user {
        Ok(user) => HttpResponse::Created().json(user),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            HttpResponse::Conflict().body("User with this email already exists")
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error while creating user: {:?}", e))
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
struct LoginQuery {
    #[schema(example = "john.doe@gmail.com")]
    email: String,

    #[schema(example = "my super password")]
    password: String,
}

#[utoipa::path(
    context_path = "/api/auth",
    request_body = LoginQuery,
    tags=["Auth"],
    responses(
        (status = 200, description = "Logged in successfully", headers(
            ("Set-Cookie", description="Set session cookie")
        )),
        (status = 401, description = "Invalid mail / password"),
        (status = 409, description = "Already logged in"),
    )
)]
#[post("/login")]
async fn login(
    user: Option<Identity>,
    login_query: web::Json<LoginQuery>,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
    request: HttpRequest,
) -> impl Responder {
    if let Some(_) = user {
        return HttpResponse::Conflict().body("Already logged in");
    }

    let mut conn = db
        .get()
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .body(format!("Diesel error occurred getting connection: {}", e))
        })
        .unwrap();

    let user = users
        .filter(email.eq(&login_query.email))
        .select((id, password))
        .first::<(Uuid, String)>(&mut conn);

    if let Err(_) = user {
        return HttpResponse::Unauthorized().body("Invalid email");
    }

    let (user_id, user_password) = user.unwrap();

    if let Err(_) = verify(&login_query.password, &user_password) {
        return HttpResponse::Unauthorized().body("Invalid password");
    }

    Identity::login(&request.extensions(), user_id.into()).unwrap();
    HttpResponse::Ok().body("Logged in")
}

#[utoipa::path(
    context_path = "/api/auth",
    tags=["Auth"],
    responses(
        (status = 200, description = "Logged out successfully", headers(
            ("Set-Cookie", description="will expires the session cookie")
        )),
        (status = 401, description = "Already logged out"),
    )
)]
#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok().body("Logged out")
}

#[utoipa::path(
    context_path = "/api/auth",
    tags=["Auth"],
    responses(
        (status = 200, description = "User information", body = User),
        (status = 500, description = "Failed to get account from db", headers(
            ("Set-Cookie", description="will expires the session cookie")
        )),
    )
)]
#[get("/me")]
async fn me(
    user: Identity,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = db
        .get()
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .body(format!("Diesel error occurred getting connection: {}", e))
        })
        .unwrap();

    //should never fail?
    let user_uuid: Uuid = user.id().unwrap().parse().unwrap();

    let user_from_db = users.find(user_uuid).first::<User>(&mut conn);

    if let Err(e) = user_from_db {
        user.logout();
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    let user_from_db = user_from_db.unwrap();

    HttpResponse::Ok().json(user_from_db)
}
