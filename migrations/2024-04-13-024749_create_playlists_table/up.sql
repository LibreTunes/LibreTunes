-- Your SQL goes here
CREATE TABLE playlists (
    id SERIAL PRIMARY KEY UNIQUE NOT NULL,
    name VARCHAR NOT NULL,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL
);

CREATE TABLE playlist_songs (
    playlist_id INTEGER REFERENCES playlists(id) ON DELETE CASCADE NOT NULL,
    song_id INTEGER REFERENCES songs(id) ON DELETE CASCADE NOT NULL,
    PRIMARY KEY (playlist_id, song_id)
);