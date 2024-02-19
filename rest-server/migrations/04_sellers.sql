CREATE TABLE IF NOT EXISTS "sellers" (
    user_id INT REFERENCES "users"(id),
    global_schedule INT REFERENCES "seller_schedules"(id),
    PRIMARY KEY (user_id)
);

CREATE TABLE IF NOT EXISTS "seller_schedules" (
    id SERIAL PRIMARY KEY,
    seller_id INT REFERENCES "sellers"(user_id),
    schedule_id INT REFERENCES "schedules"(id),
    UNIQUE (seller_id, schedule_id)
);