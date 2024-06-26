INSERT INTO "products_categories" (name)
VALUES ('Hortaliças fruto'), ('Hortaliças tuberosas'), ('Hortaliças herbáceas');

INSERT INTO "products" (name, alias, category)
VALUES
    ('ervilha', '{}', 1),
    ('tomate', '{}', 1),
    ('melancia', '{}', 1),
    ('abóbora', '{}', 1),
    ('berinjela', '{}', 1),
    ('xuxu', '{"chuchu"}', 1),
    ('maxixe', '{}', 1),
    ('abobrinha', '{"abobrinha italiana", "abobrinha verde"}', 1),
    ('alcachofra', '{}', 1),
    ('berinjela', '{}', 1);

INSERT INTO seller_products (id, product_id, seller_id, price, quantity, photos, unit, unit_quantity)
VALUES (1, 1, 3, 99.99, 10, ARRAY ['photo1.jpg', 'photo2.jpg'], 1, 100),
       (2, 2, 9, 50.00, 5, ARRAY ['photo3.jpg', 'photo4.jpg'], 1, 100),
       (3, 3, 8, 100.00, 20, ARRAY ['photo5.jpg', 'photo6.jpg'], 1, 100),
       (4, 4, 4, 75.00, 15, ARRAY ['photo7.jpg', 'photo8.jpg'], 1, 100),
       (5, 5, 5, 85.00, 25, ARRAY ['photo9.jpg', 'photo10.jpg'], 1, 100),
       (6, 6, 4, 90.00, 30, ARRAY ['photo11.jpg', 'photo12.jpg'], 1, 100),
       (7, 7, 9, 120.00, 35, ARRAY ['photo13.jpg', 'photo14.jpg'], 1, 100),
       (8, 8, 8, 150.00, 40, ARRAY ['photo15.jpg', 'photo16.jpg'], 1, 100),
       (9, 9, 9, 175.00, 45, ARRAY ['photo17.jpg', 'photo18.jpg'], 1, 100),
       (10, 10, 8, 200.00, 50, ARRAY ['photo19.jpg', 'photo20.jpg'], 1, 100);

INSERT INTO products_schedules (seller_product_id, schedule_id)
VALUES (1, 1),
       (2, 1),
       (3, 2),
       (4, 3),
       (5, 4),
       (6, 5),
       (7, 3),
       (8, 1),
       (9, 1),
       (10, 2);