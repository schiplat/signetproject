-- 015 scim_config (UI-managed bearer token; env var is a one-time seed)

CREATE TABLE scim_config (
    id BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (id),
    token_hash TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO scim_config (id, token_hash) VALUES (TRUE, NULL) ON CONFLICT (id) DO NOTHING;
