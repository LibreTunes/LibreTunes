CREATE TABLE songs (
    id SERIAL PRIMARY KEY UNIQUE NOT NULL,
    title VARCHAR NOT NULL,
    album_id INTEGER REFERENCES albums(id),
    track INTEGER,
    duration INTEGER NOT NULL,
    release_date DATE,
    storage_path VARCHAR NOT NULL,
    image_path VARCHAR
);

CREATE TABLE song_artists (
    song_id INTEGER REFERENCES songs(id) ON DELETE CASCADE NOT NULL,
    artist_id INTEGER REFERENCES artists(id) ON DELETE CASCADE NOT NULL,
    PRIMARY KEY (song_id, artist_id)
);
