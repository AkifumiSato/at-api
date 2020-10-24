CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    uid VARCHAR(48) NOT NULL UNIQUE
);

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
    info VARCHAR(100)
);
