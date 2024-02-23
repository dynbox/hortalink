CREATE TABLE IF NOT EXISTS "notifications" (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES "users"(id),
    title VARCHAR(64) NOT NULL,
    content VARCHAR(256) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "read" BOOLEAN NOT NULL DEFAULT FALSE,
    status SMALLINT NOT NULL,
    type SMALLINT NOT NULL
);

CREATE INDEX user_notification ON "notifications" (user_id);