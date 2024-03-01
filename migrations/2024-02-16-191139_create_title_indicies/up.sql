CREATE INDEX artists_name_idx ON artists USING GIST (name gist_trgm_ops);
CREATE INDEX albums_title_idx ON albums USING GIST (title gist_trgm_ops);
CREATE INDEX songs_title_idx ON songs USING GIST (title gist_trgm_ops);
