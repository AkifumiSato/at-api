CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
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