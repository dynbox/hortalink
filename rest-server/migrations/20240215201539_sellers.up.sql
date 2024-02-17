CREATE TABLE IF NOT EXISTS "sellers" (
    user_id INT REFERENCES "users"(id)
    PRIMARY KEY (user_id)
);