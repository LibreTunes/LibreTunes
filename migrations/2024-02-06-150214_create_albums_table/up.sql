CREATE TABLE albums (
    id SERIAL PRIMARY KEY UNIQUE NOT NULL,
    title VARCHAR NOT NULL,
    release_date DATE
);

-- A table to store artists for each album
-- Needed because an album can have multiple artists, but in Postgres we can't store an array of foreign keys
CREATE TABLE album_artists (
    album_id INTEGER REFERENCES albums(id) ON DELETE CASCADE NOT NULL,
    artist_id INTEGER REFERENCES artists(id) ON DELETE CASCADE NULL,
    PRIMARY KEY (album_id, artist_id)
);
