use crate::schema::translations;
use chrono::{DateTime, Utc};

#[derive(Queryable)]
pub struct Translation {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

// #[derive(Insertable)]
// #[table_name = "translations"]
// pub struct NewTranslation {

// }