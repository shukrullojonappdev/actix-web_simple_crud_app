#[macro_use]
extern  crate diesel;
extern  crate dotenv;

use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod handlers;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(handlers::index)
            .service(handlers::get_users)
            .service(handlers::get_user_by_id)
            .service(handlers::create_user)
            .service(handlers::delete_user)
    })
    .bind("127.0.0.1:1234")?
    .run()
    .await
}
