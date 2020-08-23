CREATE TABLE users (
    id INTEGER PRIMARY KEY
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
    info VARCHAR(100),
    category_id INTEGER REFERENCES action_categories (id)
);
