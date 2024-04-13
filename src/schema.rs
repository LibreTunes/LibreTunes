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
    playlist_songs (playlist_id, song_id) {
        playlist_id -> Int4,
        song_id -> Int4,
    }
}

diesel::table! {
    playlists (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
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
    }
}

diesel::joinable!(album_artists -> albums (album_id));
diesel::joinable!(album_artists -> artists (artist_id));
diesel::joinable!(playlist_songs -> playlists (playlist_id));
diesel::joinable!(playlist_songs -> songs (song_id));
diesel::joinable!(playlists -> users (user_id));
diesel::joinable!(song_artists -> artists (artist_id));
diesel::joinable!(song_artists -> songs (song_id));
diesel::joinable!(songs -> albums (album_id));

diesel::allow_tables_to_appear_in_same_query!(
    album_artists,
    albums,
    artists,
    playlist_songs,
    playlists,
    song_artists,
    songs,
    users,
);
