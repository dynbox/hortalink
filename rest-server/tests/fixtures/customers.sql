INSERT INTO customers (user_id, geolocation, address)
VALUES (2, ST_GeomFromText('POINT(-71.064544  42.28787)', 4326), '123 Main St, Boston, MA'),
       (6, ST_GeomFromText('POINT(-71.064544  42.28787)', 4326), '123 Main St, Boston, MA'),
       (7, ST_GeomFromText('POINT(-71.064544  42.28787)', 4326), '123 Main St, Boston, MA'),
       (10, ST_GeomFromText('POINT(-71.064544  42.28787)', 4326), '123 Main St, Boston, MA');