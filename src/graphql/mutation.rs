use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

pub use crate::graphql::*;

graphql_schema_from_file!("src/graphql/schema/mutation.graphql");

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_create_user(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
        translation: String,
    ) -> FieldResult<User> {
        use crate::schema::users;

        let new_user = crate::models::NewUser { translation: translation };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<crate::models::User>(&executor.context().db_con)
            .map(Into::into)
            .map_err(Into::into)
    }

    fn field_create_post(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
        user_id: i32,
        title: String,
        body: Option<String>
    ) -> FieldResult<Post> {
        use crate::schema::posts;

        let new_post = crate::models::NewPost { 
            user_id: user_id,
            title: title,
            body: body,
            created_at: Utc::now()
        };

        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result::<crate::models::Post>(&executor.context().db_con)
            .map(Into::into)
            .map_err(Into::into)
    }

    fn field_delete_post(
        &self,
        executor: &Executor<'_, Context>,
        post_id: i32
    ) -> FieldResult<i32> {
        use crate::schema::posts::dsl::*;

        diesel::delete(posts.filter(id.eq(post_id)))
            .execute(&executor.context().db_con)
            .map(|rownum| rownum as i32)
            .map_err(Into::into)
            
    }
}