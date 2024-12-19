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
    friend_requests (from_id, to_id) {
        created_at -> Timestamp,
        from_id -> Int4,
        to_id -> Int4,
    }
}

diesel::table! {
    friendships (friend_1_id, friend_2_id) {
        created_at -> Timestamp,
        friend_1_id -> Int4,
        friend_2_id -> Int4,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
        owner_id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    song_artists (song_id, artist_id) {
        song_id -> Int4,
        artist_id -> Int4,
    }
}

diesel::table! {
    song_dislikes (song_id, user_id) {
        song_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    song_history (id) {
        id -> Int4,
        user_id -> Int4,
        date -> Timestamp,
        song_id -> Int4,
    }
}

diesel::table! {
    song_likes (song_id, user_id) {
        song_id -> Int4,
        user_id -> Int4,
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
        added_date -> Date,
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
diesel::joinable!(playlist_songs -> playlists (playlist_id));
diesel::joinable!(playlist_songs -> songs (song_id));
diesel::joinable!(playlists -> users (owner_id));
diesel::joinable!(song_artists -> artists (artist_id));
diesel::joinable!(song_artists -> songs (song_id));
diesel::joinable!(song_dislikes -> songs (song_id));
diesel::joinable!(song_dislikes -> users (user_id));
diesel::joinable!(song_history -> songs (song_id));
diesel::joinable!(song_history -> users (user_id));
diesel::joinable!(song_likes -> songs (song_id));
diesel::joinable!(song_likes -> users (user_id));
diesel::joinable!(songs -> albums (album_id));

diesel::allow_tables_to_appear_in_same_query!(
    album_artists,
    albums,
    artists,
    friend_requests,
    friendships,
    playlist_songs,
    playlists,
    song_artists,
    song_dislikes,
    song_history,
    song_likes,
    songs,
    users,
);
