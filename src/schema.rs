// @generated automatically by Diesel CLI.

diesel::table! {
    album_artists (album_id, artist_id) {
        album_id -> Int4,
        artist_id -> Int4,
    }
}

diesel::table! {
    albums (id) {
        id -> Int4,
        title -> Varchar,
        release_date -> Nullable<Date>,
    }
}

diesel::table! {
    artists (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(album_artists -> albums (album_id));
diesel::joinable!(album_artists -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(
    album_artists,
    albums,
    artists,
    users,
);
