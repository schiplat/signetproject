-- 008 phone uniqueness (non-null values only)

CREATE UNIQUE INDEX users_phone_key ON users (phone) WHERE phone IS NOT NULL;
