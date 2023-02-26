use crate::schema::components;
use chrono::{DateTime, Utc};

#[derive(Queryable)]
pub struct Component {
    pub id: i32,
    pub name: String,
    pub translations_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

// #[derive(Insertable)]
// #[table_name = "components"]
// pub struct NewComponent {
//   pub name: String,
//   pub translations_id: i32,
//   pub created_at: DateTime<Utc>,
//   pub updated_at: DateTime<Utc>
// }
