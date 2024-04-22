CREATE TABLE IF NOT EXISTS "products"
(
    id    SERIAL PRIMARY KEY,
    name  VARCHAR(32)  NOT NULL,
    photo VARCHAR(256) NOT NULL,
    alias VARCHAR(256)[]
);

CREATE TABLE IF NOT EXISTS "seller_products"
(
    id          SERIAL PRIMARY KEY,
    product_id  INT REFERENCES "products" (id),
    seller_id   INT REFERENCES "sellers" (user_id),
    price       NUMERIC(7, 2)  NOT NULL,
    quantity    SMALLINT,
    photos      VARCHAR(256)[] NOT NULL,
    schedule_id INT REFERENCES "schedules" (id)
);

CREATE INDEX seller_product_id ON "seller_products" (seller_id);
CREATE INDEX seller_product_price ON "seller_products" (price);
