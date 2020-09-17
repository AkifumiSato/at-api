CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    uid VARCHAR(48) UNIQUE
);

CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users (id),
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  published_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id),
    name VARCHAR NOT NULL,
    slug VARCHAR NOT NULL
);

CREATE TABLE posts_tags (
    post_id INTEGER NOT NULL REFERENCES posts (id),
    tag_id INTEGER NOT NULL REFERENCES tags (id),
    PRIMARY KEY (post_id, tag_id)
);

CREATE INDEX posts_tags_post_id_idx ON posts_tags (post_id);
CREATE INDEX posts_tags_tag_name_idx ON posts_tags (tag_id);

CREATE TABLE action_categories (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id),
    name VARCHAR(20) NOT NULL
);

CREATE TABLE action_records (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id),
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    info VARCHAR(100),
    category_id INTEGER REFERENCES action_categories (id)
);
