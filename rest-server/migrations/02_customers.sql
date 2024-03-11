CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE IF NOT EXISTS "customers" (
    user_id INT REFERENCES "users"(id),
    geolocation GEOMETRY(Point, 4326) NOT NULL,
    address TEXT NOT NULL,
    PRIMARY KEY (user_id)
);