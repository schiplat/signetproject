-- 006 login security, sessions, groups, oauth consents

-- Login brute-force protection
ALTER TABLE users
    ADD COLUMN failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN locked_until TIMESTAMPTZ;

-- Password policy: last change + history for reuse prevention
ALTER TABLE users
    ADD COLUMN password_changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ADD COLUMN groups TEXT[] NOT NULL DEFAULT '{}';

CREATE TABLE password_history (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX password_history_user_id_idx ON password_history(user_id);

-- Session metadata for the session-management UI
ALTER TABLE sessions
    ADD COLUMN ip TEXT,
    ADD COLUMN user_agent TEXT,
    ADD COLUMN last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

CREATE INDEX sessions_user_id_created_idx ON sessions(user_id, created_at DESC);

-- OIDC consent (remembered per user + client)
CREATE TABLE oauth_consents (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_id TEXT NOT NULL REFERENCES client_apps(client_id) ON DELETE CASCADE,
    scopes TEXT NOT NULL,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, client_id)
);
