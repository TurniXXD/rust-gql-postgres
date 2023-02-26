use crate::schema::translations_packs;
use chrono::{DateTime, Utc};

#[derive(Queryable)]
pub struct TranslationsPack {
    pub id: i32,
    pub lang: String,
    pub translation: String,
    pub translation_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Insertable)]
#[table_name = "translations_packs"]
pub struct NewTranslationsPack {
  pub lang: String,
  pub translation: String,
  pub translation_id: i32,
}