-- 012 SCIM v2 support

ALTER TABLE users ADD COLUMN external_id TEXT;

CREATE UNIQUE INDEX users_external_id_key ON users(external_id) WHERE external_id IS NOT NULL;

CREATE TABLE scim_groups (
    id UUID PRIMARY KEY,
    display_name TEXT NOT NULL UNIQUE,
    external_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
