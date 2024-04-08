INSERT INTO "seller_rating" (author_id, seller_id, rating, tags)
VALUES (2, 3, 5, ARRAY [1, 2]),
       (7, 8, 5, ARRAY [2, 3]),
       (6, 4, 3, ARRAY [3, 4]),
       (2, 9, 5, ARRAY [4, 5]),
       (10, 8, 4, ARRAY [1, 3]),
       (6, 5, 5, ARRAY [2, 4]);

INSERT INTO "seller_product_ratings" (seller_product_id, author_id, rating, content)
VALUES (1, 6, 5, 'Great product, would buy again.'),
       (8, 2, 4, 'Good quality, but a bit overpriced.'),
       (3, 7, 3, 'Could be better.'),
       (8, 7, 2, 'Terrible product, don''t waste your money.'),
       (5, 10, 5, 'Excellent service and product.'),
       (6, 6, 4, 'Decent product, could be better.'),
       (8, 6, 1, 'Never again, worst experience.'),
       (3, 6, 5, 'Absolutely fantastic, highly recommended.'),
       (8, 10, 4, 'Good product, but not the best.');