CREATE TABLE IF NOT EXISTS "cart" (
    inventory_id INT REFERENCES "inventories"(product_id),
    customer_id INT REFERENCES "customers"(user_id),
    status SMALLINT NOT NULL DEFAULT 1,
    withdrawn TIMESTAMP NOT NULL,
    UNIQUE(inventory_id, customer_id),
    PRIMARY KEY inventory_id
);

CREATE INDEX customer_cart ON "cart" (customer_id);