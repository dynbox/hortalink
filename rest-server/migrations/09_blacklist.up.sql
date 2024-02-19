CREATE TABLE IF NOT EXISTS "blacklist" (
    user_id INT REFERENCES "users"(id),
    end_time TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id)
);