CREATE TABLE friend_requests (
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	from_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL,
	to_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL,
	PRIMARY KEY (from_id, to_id)
);

CREATE INDEX outgoing_friend_requests_idx ON friend_requests(from_id);
CREATE INDEX incoming_friend_requests_idx ON friend_requests(to_id);

CREATE TABLE friendships (
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	friend_1_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL,
	friend_2_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL,
	PRIMARY KEY (friend_1_id, friend_2_id)
);

CREATE INDEX friendships_friend_1_idx ON friendships(friend_1_id);
CREATE INDEX friendships_friend_2_idx ON friendships(friend_2_id);
