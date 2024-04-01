CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE IF NOT EXISTS "customers"
(
    user_id INT REFERENCES "users" (id),
    PRIMARY KEY (user_id)
);