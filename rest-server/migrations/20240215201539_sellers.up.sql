CREATE TABLE IF NOT EXISTS "sellers" (
    user_id INT REFERENCES "users"(id),
    global_schedule INT REFERENCES "schedules"(id),
    PRIMARY KEY (user_id)
);