INSERT INTO schedules (geolocation, address, start_time, end_time, day_of_week, seller_id)
VALUES (ST_GeomFromText('POINT(0  0)', 4674), '123 Main St', '08:00:00', '17:00:00', 1, 3),
       (ST_GeomFromText('POINT(0.45  10)', 4674), '456 Elm St', '09:00:00', '18:00:00', 2, 8),
       (ST_GeomFromText('POINT(0.90  20)', 4674), '789 Oak St', '10:00:00', '19:00:00', 3, 9),
       (ST_GeomFromText('POINT(1.35  30)', 4674), '321 Pine St', '11:00:00', '20:00:00', 4, 3),
       (ST_GeomFromText('POINT(1.80  40)', 4674), '654 Maple St', '12:00:00', '21:00:00', 5, 8);