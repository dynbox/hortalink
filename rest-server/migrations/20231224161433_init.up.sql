CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS
    "users" (
        id UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
        name VARCHAR(64),
        email VARCHAR(255) NOT NULL UNIQUE,
        phone INTEGER UNIQUE,
        account_type INTEGER DEFAULT 2,
        password VARCHAR(255),
        address varchar(255),
        avatar varchar(64),
        bio varchar(511),
        access_token text
    );

CREATE SCHEMA IF NOT EXISTS "tower_sessions";

CREATE TABLE IF NOT EXISTS
    "tower_sessions"."sessions" (
        id text PRIMARY KEY NOT NULL,
        data bytea NOT NULL,
        expiry_date timestamptz NOT NULL
    );