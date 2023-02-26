use std::convert::From;
use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use futures01::future::Future;

use juniper::http::playground::playground_source;
use juniper::{http::GraphQLRequest};
use juniper_from_schema::graphql_schema_from_file;

use diesel::prelude::*;

use itertools::Itertools;

use crate::{DbPool};

use chrono::{Utc};

pub use context::*;
pub use query::*;
pub use mutation::*;
pub use user::*;
pub use post::*;
pub use translation::*;

pub mod context;
pub mod query;
pub mod mutation;
pub mod user;
pub mod post;
pub mod translation;

graphql_schema_from_file!("src/graphql/schema/schema.graphql");

fn playground() -> HttpResponse {
    let html = playground_source("/query");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    db_pool: web::Data<DbPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let ctx = Context {
        db_con: db_pool.get().unwrap(),
    };

    web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|data| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(data))
    })
}

pub fn register(config: &mut web::ServiceConfig) {
    let schema = std::sync::Arc::new(Schema::new(Query, Mutation));

    config
        .data(schema)
        .route("/query", web::post().to_async(graphql))
        .route("/graphql", web::get().to(playground));
}