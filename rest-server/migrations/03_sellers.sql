CREATE TABLE IF NOT EXISTS "sellers" (
    user_id INT REFERENCES "users"(id),
    bio VARCHAR(512),
    PRIMARY KEY (user_id)
);