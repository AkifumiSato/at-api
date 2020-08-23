CREATE TABLE users (
    id INTEGER PRIMARY KEY
);

CREATE TABLE action_records (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id),
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    info VARCHAR(100)
);

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id),
    name VARCHAR(20)
);

CREATE TABLE action_records_categories (
    record_id INTEGER NOT NULL REFERENCES action_records (id),
    category_id INTEGER NOT NULL REFERENCES categories (id),
    PRIMARY KEY (record_id, category_id)
);
