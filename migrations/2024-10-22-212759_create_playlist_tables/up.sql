CREATE TABLE playlists (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    owner_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    name TEXT NOT NULL
);

CREATE INDEX playlists_owner_idx ON playlists(owner_id);

CREATE TABLE playlist_songs (
    playlist_id INTEGER REFERENCES playlists(id) ON DELETE CASCADE NOT NULL,
    song_id INTEGER REFERENCES songs(id) ON DELETE CASCADE NOT NULL,
    PRIMARY KEY (playlist_id, song_id)
);

CREATE INDEX playlist_songs_playlist_idx ON playlist_songs(playlist_id);
