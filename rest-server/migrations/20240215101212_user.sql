CREATE TABLE IF NOT EXISTS "users" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    username CHAR(16) NOT NULL UNIQUE,
    email VARCHAR(256) NOT NULL UNIQUE,
    avatar VARCHAR(256),
    password TEXT,
    account_type INTEGER DEFAULT 2,
    address varchar(256),
    bio varchar(512),
    permissions INT NOT NULL DEFAULT 0,
    access_token TEXT
);

CREATE INDEX user_email ON "users" (email);
CREATE INDEX user_username ON "users" (username);