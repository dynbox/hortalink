CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE IF NOT EXISTS "schedules" (
    id SERIAL PRIMARY KEY,
    seller_id INT REFERENCES sellers(id),
    geolocation GEOMETRY(Point, 4326) NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    day_of_week SMALLINT NOT NULL
);

CREATE INDEX seller_schedule ON "schedules"(seller_id);
CREATE INDEX schedule_duration ON "schedules"(day_of_week, start_time, end_time);
CREATE INDEX schedules_geolocation ON schedules USING gist(geolocation);