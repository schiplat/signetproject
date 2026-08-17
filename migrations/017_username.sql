-- 017 optional username login identifier

ALTER TABLE users ADD COLUMN username TEXT;

CREATE UNIQUE INDEX users_username_key ON users(username) WHERE username IS NOT NULL;
