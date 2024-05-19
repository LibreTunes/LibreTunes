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
        image_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    artists (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    song_artists (song_id, artist_id) {
        song_id -> Int4,
        artist_id -> Int4,
    }
}

diesel::table! {
    songs (id) {
        id -> Int4,
        title -> Varchar,
        album_id -> Nullable<Int4>,
        track -> Nullable<Int4>,
        duration -> Int4,
        release_date -> Nullable<Date>,
        storage_path -> Varchar,
        image_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        admin -> Bool,
    }
}

diesel::joinable!(album_artists -> albums (album_id));
diesel::joinable!(album_artists -> artists (artist_id));
diesel::joinable!(song_artists -> artists (artist_id));
diesel::joinable!(song_artists -> songs (song_id));
diesel::joinable!(songs -> albums (album_id));

diesel::allow_tables_to_appear_in_same_query!(
    album_artists,
    albums,
    artists,
    song_artists,
    songs,
    users,
);
