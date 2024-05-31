CREATE TABLE IF NOT EXISTS "followers"
(
    id          BIGSERIAL PRIMARY KEY,
    seller_id   INT REFERENCES "sellers" (user_id),
    customer_id INT REFERENCES "customers" (user_id),
    UNIQUE (seller_id, customer_id)
);

CREATE INDEX customers_id ON "followers" (customer_id);