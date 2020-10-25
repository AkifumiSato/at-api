CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    uid VARCHAR(48) NOT NULL UNIQUE
);

CREATE TABLE attendance_records (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id),
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    break_time INTEGER NOT NULL
);
