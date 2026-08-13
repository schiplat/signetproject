ALTER TABLE audit_logs
    ADD COLUMN ip TEXT;

CREATE INDEX audit_logs_action_created_at_idx ON audit_logs(action, created_at DESC);

ALTER TABLE client_apps
    ADD COLUMN ip_allowlist_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    ADD COLUMN allowed_cidrs TEXT[] NOT NULL DEFAULT '{}';

-- Existing clients remain unrestricted until explicitly hardened.
UPDATE client_apps SET ip_allowlist_enabled = FALSE;
