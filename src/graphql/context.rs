use crate::{DbCon};

pub struct Context {
    pub db_con: DbCon,
}
impl juniper::Context for Context {}

