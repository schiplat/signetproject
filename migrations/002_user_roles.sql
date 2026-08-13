-- Replace boolean is_admin with role: admin | manager | member
ALTER TABLE users
    ADD COLUMN role TEXT NOT NULL DEFAULT 'member'
        CHECK (role IN ('admin', 'manager', 'member'));

UPDATE users SET role = 'admin' WHERE is_admin = TRUE;

ALTER TABLE users DROP COLUMN is_admin;

CREATE INDEX users_role_idx ON users(role);
