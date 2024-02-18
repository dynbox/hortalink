CREATE TABLE IF NOT EXISTS "inventory_rating" (
    id SERIAL PRIMARY KEY,
    inventory_id INT REFERENCES "seller_products"(id),
    customer_id INT REFERENCES "customers"(user_id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    was_edited BOOLEAN NOT NULL DEFAULT FALSE,
    rating SMALLINT NOT NULL,
    content VARCHAR(256) NOT NULL,
    UNIQUE(inventory_id, customer_id)
);

CREATE INDEX inventory_rating ON "product_rating"(inventory_id, rating);

CREATE TABLE IF NOT EXISTS "customer_rating" (
    id SERIAL PRIMARY KEY,
    author_id INT REFERENCES "sellers"(product_id),
    customer_id INT REFERENCES "customers"(user_id),
    rating SMALLINT NOT NULL,
    UNIQUE(author_id, customer_id)
);

CREATE INDEX customer_rating_id ON "customer_rating"(customer_id);

CREATE TABLE IF NOT EXISTS "seller_rating" (
    id SERIAL PRIMARY KEY,
    author_id INT REFERENCES "customers"(product_id),
    seller_id INT REFERENCES "sellers"(user_id),
    rating SMALLINT NOT NULL,
    UNIQUE(author_id, seller_id)
);

CREATE INDEX seller_rating_id ON "customer_rating"(seller_id);