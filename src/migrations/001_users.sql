CREATE TABLE IF NOT EXISTS testing.users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_username ON testing.users(username);
