use actix_web::{web, get, post, delete, Responder, HttpResponse, Result};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::RunQueryDsl;
use diesel::dsl::{delete, insert_into};
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    age: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub name: String,
    pub age: u32
}

#[get("/users")]
async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse> {
    Ok(web::block(move || get_all_users(db))
       .await
        .map(|user| HttpResponse::Ok().json(user))
    )
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

#[get("/users/{id}")]
async fn get_user_by_id(id: web::Path<String>) -> Result<impl Responder> {
    Ok(format!("get user by id {}", id))
}

#[post("/users")]
async fn create_user(mut payload: web::Payload) -> Result<HttpResponse> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }
    let obj = serde_json::from_slice::<User>(&body)?;
    Ok(HttpResponse::Ok().json(obj))
}

#[delete("/users/{id}")]
async fn delete_user(id: web::Path<String>) -> Result<impl Responder> {
    Ok(format!("deleted user {}", id))
}
