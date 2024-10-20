CREATE TABLE IF NOT EXISTS "cart"
(
    id                SERIAL PRIMARY KEY,
    seller_product_id INT REFERENCES "seller_products" (id),
    customer_id       INT REFERENCES "customers" (user_id),
    status            SMALLINT  NOT NULL DEFAULT 1,
    withdrawn         TIMESTAMP NOT NULL,
    amount            INT       NOT NULL,
    UNIQUE (seller_product_id, customer_id)
);

CREATE INDEX customer_cart ON "cart" (customer_id);