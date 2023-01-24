-- Add migration script here in postgresql
CREATE TABLE IF NOT EXISTS "messages" (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    content VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "users"(id)
);