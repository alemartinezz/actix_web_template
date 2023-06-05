use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::User;
use bcrypt::{DEFAULT_COST, hash};

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<RegisterUser>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    let conn = pool.get().expect("Failed to get db connection from pool");

    let hashed_password = hash(&new_user.password, DEFAULT_COST).unwrap();

    diesel::insert_into(users)
        .values((email.eq(&new_user.email), password.eq(hashed_password)))
        .execute(&conn)
        .expect("Error saving new user");

    HttpResponse::Ok().body("User created")
}
