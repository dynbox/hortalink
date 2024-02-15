CREATE SCHEMA IF NOT EXISTS "tower_sessions";

CREATE TABLE IF NOT EXISTS "tower_sessions"."sessions" (
    id text PRIMARY KEY NOT NULL,
    data bytea NOT NULL,
    expiry_date timestamptz NOT NULL
);