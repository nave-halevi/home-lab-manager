-- Add migration script here
ALTER TABLE users ADD COLUMN total_score INT NOT NULL DEFAULT 0;