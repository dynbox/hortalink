CREATE TABLE IF NOT EXISTS "seller_product_ratings"
(
    id                BIGSERIAL PRIMARY KEY,
    seller_product_id BIGINT REFERENCES "seller_products" (id),
    author_id         INT REFERENCES "customers" (user_id),
    created_at        TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    was_edited        BOOLEAN      NOT NULL DEFAULT FALSE,
    rating            SMALLINT     NOT NULL,
    content           VARCHAR(256) NOT NULL,
    UNIQUE (seller_product_id, author_id)
);

CREATE INDEX seller_product_rating ON "seller_product_ratings" (seller_product_id, rating);