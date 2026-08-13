-- 007 phone field + audit log client info (user-agent / browser / os)

ALTER TABLE users
    ADD COLUMN phone TEXT;

ALTER TABLE audit_logs
    ADD COLUMN user_agent TEXT,
    ADD COLUMN browser TEXT,
    ADD COLUMN os TEXT;
