CREATE TABLE IF NOT EXISTS "messages"
(
    id          BIGSERIAL PRIMARY KEY,
    author_id   INT REFERENCES "users" (id),
    received_id INT REFERENCES "users" (id),
    content     TEXT      NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX dm ON "messages" (author_id, received_id);