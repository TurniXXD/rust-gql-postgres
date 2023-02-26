// @generated automatically by Diesel CLI.

diesel::table! {
    components (id) {
        id -> Int4,
        name -> Text,
        translations_id -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        body -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    translations (id) {
        id -> Int4,
    }
}

diesel::table! {
    translations_packs (id) {
        id -> Int4,
        lang -> Text,
        translation -> Text,
        translation_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        translation -> Varchar,
    }
}

diesel::joinable!(components -> translations (translations_id));
diesel::joinable!(posts -> users (user_id));
diesel::joinable!(translations_packs -> translations (translation_id));

diesel::allow_tables_to_appear_in_same_query!(
    components,
    posts,
    translations,
    translations_packs,
    users,
);
