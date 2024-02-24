INSERT INTO schedules (geolocation, address, start_time, end_time, day_of_week) VALUES
    (ST_GeomFromText('POINT(0  0)',  4326), '123 Main St', '08:00:00', '17:00:00',  1),
    (ST_GeomFromText('POINT(10  10)',  4326), '456 Elm St', '09:00:00', '18:00:00',  2),
    (ST_GeomFromText('POINT(20  20)',  4326), '789 Oak St', '10:00:00', '19:00:00',  3),
    (ST_GeomFromText('POINT(30  30)',  4326), '321 Pine St', '11:00:00', '20:00:00',  4),
    (ST_GeomFromText('POINT(40  40)',  4326), '654 Maple St', '12:00:00', '21:00:00',  5);
    
INSERT INTO seller_schedules (seller_id, schedule_id) VALUES
    (3,  1),
    (4,  2),
    (5,  3),
    (8,  4),
    (8,  5);