use crate::schema::posts;
use chrono::{DateTime, Utc};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>
}
