#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

use dotenv::dotenv;
use std::env;

pub mod graphql;
pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbCon = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

fn main() {
    let db_pool = create_db_pool();
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(Cors::new())
            .configure(graphql::register)
            .default_service(web::to(|| "404"))
    })
    .bind(addr)
    .unwrap()
    .run()
    .unwrap();
}

fn create_db_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    r2d2::Pool::builder()
        .max_size(3)
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("failed to create db connection pool")
}