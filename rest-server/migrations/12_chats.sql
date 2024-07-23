CREATE TABLE IF NOT EXISTS "chats"
(
    id    BIGSERIAL PRIMARY KEY,
    user1 INT REFERENCES "users" (id),
    user2 INT REFERENCES "users" (id)
);

CREATE INDEX dm ON "chats" (user1, user2);