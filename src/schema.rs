// @generated automatically by Diesel CLI.

diesel::table! {
    vote (id) {
        id -> Uuid,
        challenge -> Bytea,
        responses -> Bytea,
        encrypted -> Bytea,
        key_image -> Bytea,
        nonce -> Bytea,
        key -> Nullable<Bytea>,
        voting_id -> Uuid,
    }
}

diesel::table! {
    voting (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Text,
        options -> Array<Nullable<Text>>,
        active -> Bool,
    }
}

diesel::joinable!(vote -> voting (voting_id));

diesel::allow_tables_to_appear_in_same_query!(
    vote,
    voting,
);
