-- Add migration script here in postgresql
CREATE TABLE IF NOT EXISTS "users_uuid" (
    user_id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "users"(id)
);
