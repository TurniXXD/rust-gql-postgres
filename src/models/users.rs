use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub translation: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub translation: String,
}