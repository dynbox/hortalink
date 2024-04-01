CREATE TABLE IF NOT EXISTS "sellers"
(
    user_id INT REFERENCES "users" (id),
    bio     VARCHAR(256),
    PRIMARY KEY (user_id)
);