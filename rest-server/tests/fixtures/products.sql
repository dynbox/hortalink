INSERT INTO products (name, photo, alias)
VALUES ('Product 1', 'photo1.jpg', ARRAY ['alias1', 'alias2']),
       ('Product 2', 'photo2.jpg', ARRAY ['alias3', 'alias4']),
       ('Product 3', 'photo3.jpg', ARRAY ['alias5', 'alias6']),
       ('Product 4', 'photo4.jpg', ARRAY ['alias7', 'alias8']),
       ('Product 5', 'photo5.jpg', ARRAY ['alias9', 'alias10']),
       ('Product 6', 'photo6.jpg', ARRAY ['alias11', 'alias12']),
       ('Product 7', 'photo7.jpg', ARRAY ['alias13', 'alias14']),
       ('Product 8', 'photo8.jpg', ARRAY ['alias15', 'alias16']),
       ('Product 9', 'photo9.jpg', ARRAY ['alias17', 'alias18']),
       ('Product 10', 'photo10.jpg', ARRAY ['alias19', 'alias20']);

INSERT INTO seller_products (product_id, seller_id, price, quantity, photos)
VALUES (1, 3, 99.99, 10, ARRAY ['photo1.jpg', 'photo2.jpg']),
       (2, 9, 50.00, 5, ARRAY ['photo3.jpg', 'photo4.jpg']),
       (3, 8, 100.00, 20, ARRAY ['photo5.jpg', 'photo6.jpg']),
       (4, 4, 75.00, 15, ARRAY ['photo7.jpg', 'photo8.jpg']),
       (5, 5, 85.00, 25, ARRAY ['photo9.jpg', 'photo10.jpg']),
       (6, 4, 90.00, 30, ARRAY ['photo11.jpg', 'photo12.jpg']),
       (7, 9, 120.00, 35, ARRAY ['photo13.jpg', 'photo14.jpg']),
       (8, 8, 150.00, 40, ARRAY ['photo15.jpg', 'photo16.jpg']),
       (9, 9, 175.00, 45, ARRAY ['photo17.jpg', 'photo18.jpg']),
       (10, 8, 200.00, 50, ARRAY ['photo19.jpg', 'photo20.jpg']);