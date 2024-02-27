CREATE TABLE IF NOT EXISTS "users" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    username CHAR(16) NOT NULL UNIQUE,
    email VARCHAR(256) NOT NULL UNIQUE,
    avatar VARCHAR(256),
    password TEXT,
    roles SMALLINT[] NOT NULL,
    phone CHAR(11),
    access_token TEXT
);

CREATE INDEX user_email ON "users"(email);
CREATE INDEX user_username ON "users"(username);