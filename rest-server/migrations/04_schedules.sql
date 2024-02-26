CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE IF NOT EXISTS "schedules" (
    id SERIAL PRIMARY KEY,
    geolocation GEOMETRY(Point, 4326) NOT NULL,
    address VARCHAR(256) NOT NULL,
    start_time TIME WITHOUT TIME ZONE NOT NULL,
    end_time TIME WITHOUT TIME ZONE NOT NULL,
    day_of_week SMALLINT NOT NULL
);

CREATE INDEX schedule_duration ON "schedules"(day_of_week, start_time, end_time);
CREATE INDEX schedule_geolocation ON "schedules" USING gist(geolocation);

CREATE TABLE IF NOT EXISTS "seller_schedules" (
    id SERIAL PRIMARY KEY,
    seller_id INT REFERENCES "sellers"(user_id),
    schedule_id INT REFERENCES "schedules"(id),
    UNIQUE (seller_id, schedule_id)
);