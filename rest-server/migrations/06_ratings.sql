CREATE TABLE IF NOT EXISTS "seller_product_ratings" (
    id SERIAL PRIMARY KEY,
    seller_product_id INT REFERENCES "seller_products"(id),
    customer_id INT REFERENCES "customers"(user_id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    was_edited BOOLEAN NOT NULL DEFAULT FALSE,
    rating SMALLINT NOT NULL,
    content VARCHAR(256) NOT NULL,
    UNIQUE(seller_product_id, customer_id)
);

CREATE INDEX seller_product_rating ON "seller_product_ratings"(seller_product_id, rating);

CREATE TABLE IF NOT EXISTS "customer_ratings" (
    id SERIAL PRIMARY KEY,
    author_id INT REFERENCES "sellers"(user_id),
    customer_id INT REFERENCES "customers"(user_id),
    rating SMALLINT NOT NULL,
    tags SMALLINT[],
    UNIQUE(author_id, customer_id)
);

CREATE INDEX customer_rating_id ON "customer_ratings"(customer_id);

CREATE TABLE IF NOT EXISTS "seller_rating" (
    id SERIAL PRIMARY KEY,
    author_id INT REFERENCES "customers"(user_id),
    seller_id INT REFERENCES "sellers"(user_id),
    rating SMALLINT NOT NULL,
    tags SMALLINT[],
    UNIQUE(author_id, seller_id)
);

CREATE INDEX seller_rating_id ON "seller_rating"(seller_id);