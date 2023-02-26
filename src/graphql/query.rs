pub use crate::graphql::*;
use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/graphql/schema/query.graphql");

pub struct Query;

impl QueryFields for Query {
    fn field_users(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<Vec<User>> {
        use crate::schema::users;

        users::table
            .load::<crate::models::User>(&executor.context().db_con)
            .and_then(|users| Ok(users.into_iter().map_into().collect()))
            .map_err(Into::into)
    }

    fn field_posts(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
    ) -> FieldResult<Vec<Post>> {
        use crate::schema::posts;

        posts::table
            .load::<crate::models::Post>(&executor.context().db_con)
            .and_then(|posts| Ok(posts.into_iter().map_into().collect()))
            .map_err(Into::into)
    }

    fn field_user_by_id(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
        id: i32,
    ) -> FieldResult<User> {
        use crate::schema::users::dsl::users;

        users
            .find(id)
            .first::<crate::models::User>(&executor.context().db_con)
            .map(Into::into)
            .map_err(Into::into)
    }

    fn field_post_by_id(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
        id: i32,
    ) -> FieldResult<Post> {
        use crate::schema::posts::dsl::posts;

        posts
            .find(id)
            .first::<crate::models::Post>(&executor.context().db_con)
            .map(Into::into)
            .map_err(Into::into)
    }

    fn field_get_all_static_translations(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Translation, Walked>,
        input: GetAllStaticTranslationsInput,
    ) -> FieldResult<Vec<Translation>> {
        use crate::schema::components::dsl::components;
        use crate::schema::components::{
            name as component_name_col, translations_id as components_translation_id,
        };
        use crate::schema::translations::dsl::translations;
        use crate::schema::translations::id as translations_id;
        use crate::schema::translations_packs::dsl::translations_packs;
        use crate::schema::translations_packs::{
            lang as lang_col, translation_id as translations_translation_id_col,
        };

        let components_found = components
            .select((component_name_col, components_translation_id))
            .load::<crate::models::Component>(&executor.context().db_con);

        let results = components_found?
            .into_iter()
            .map(|component| {
                let translation_rel = translations
                    .filter(translations_id.eq(component.translations_id))
                    .first::<crate::models::Translation>(&executor.context().db_con)
                    .expect("Error finding translation by ID");

                let translation = translations_packs
                    .filter(
                        translations_translation_id_col
                            .eq(translation_rel.id)
                            .and(lang_col.eq(input.locale)),
                    )
                    .first::<crate::models::TranslationsPack>(&executor.context().db_con)
                    .expect("Error finding translation pack");

                return Translation {
                    uid_path: component.name,
                    translation: translation.translation,
                };
            })
            .collect::<Vec<Translation>>();

        Ok(results);

        // for component in components_found {
        //     let translation_rel = translations
        //         .find(component.translation_id)
        //         .first::<crate::models::Component>(&executor.context().db_con)
        //         .map(Into::into)
        //         .map_err(Into::into);

        //     let translation = translations_packs
        //         //.find(translation_rel.translation_id, locale)
        //         .filter(translations_translation_id_col.eq(translation_rel.translation_id).and(lang_col.eq(input.locale)))
        //         .first::<crate::models::Component>(&executor.context().db_con)
        //         .map(Into::into)
        //         .map_err(Into::into);

        //     println!("Component {}: {}", component.id, component.name);

        //     let results = all_translations
        //     .into_iter()
        //     .map(|(uid_path, translation, name)| (Translation { id: 0, component_id, uid_path, translation }, Component { id: 0, name }))
        //     .collect::<Vec<_>>();

        //     return results
        // }
        // translations::table
        //     .load::<crate::models::User>(&executor.context().db_con)
        //     .and_then(|users| Ok(users.into_iter().map_into().collect()))
        //     .map_err(Into::into)
    }
}
