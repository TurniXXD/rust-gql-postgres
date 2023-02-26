use std::convert::From;
use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

use chrono::{DateTime, Utc};

pub use crate::graphql::context::*;

graphql_schema_from_file!("src/graphql/schema/post.graphql");

pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>
}


impl PostFields for Post {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<juniper::ID> {
        Ok(juniper::ID::new(self.id.to_string()))
    }

    fn field_user_id(&self, _: &Executor<'_, Context>) -> FieldResult<juniper::ID> {
        Ok(juniper::ID::new(self.user_id.to_string()))
    }

    fn field_title(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.title)
    }

    fn field_body(&self, _: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.body)
    }

    fn field_created_at(&self, _: &Executor<'_, Context>) -> FieldResult<&DateTime<Utc>> {
        Ok(&self.created_at)
    }
}

impl From<crate::models::Post> for Post {
    fn from(post: crate::models::Post) -> Self {
        Self {
            id: post.id,
            user_id: post.user_id,
            title: post.title,
            body: post.body,
            created_at: post.created_at
        }
    }
}
