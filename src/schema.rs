// @generated automatically by Diesel CLI.

diesel::table! {
    hits (id) {
        id -> Integer,
        ip_hash -> Binary,
        slug -> Text,
        timestamp -> TimestamptzSqlite,
    }
}
