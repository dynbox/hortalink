INSERT INTO schedules (geolocation, address, start_time, end_time, day_of_week, seller_id)
VALUES (ST_GeomFromText('POINT(-46.6333 -23.5505)', 4674), 'Mercado Municipal, São Paulo', '08:00', '17:00', 1, 2),
       (ST_GeomFromText('POINT(-46.6333 -23.5505)', 4674), 'Mercado Municipal, São Paulo', '08:00', '17:00', 3, 2),

       (ST_GeomFromText('POINT(-43.1729 -22.9068)', 4674), 'Feira Livre Copacabana, Rio de Janeiro', '07:00', '13:00',
        6, 3),
       (ST_GeomFromText('POINT(-43.1729 -22.9068)', 4674), 'Feira Livre Copacabana, Rio de Janeiro', '07:00', '13:00',
        7, 3),

       (ST_GeomFromText('POINT(-47.8822 -15.7942)', 4674), 'Ceasa DF, Brasília', '06:00', '14:00', 2, 5),
       (ST_GeomFromText('POINT(-47.8822 -15.7942)', 4674), 'Ceasa DF, Brasília', '06:00', '14:00', 4, 5),

       (ST_GeomFromText('POINT(-48.5493 -27.5936)', 4674), 'Mercado Público, Florianópolis', '09:00', '18:00', 5, 9),
       (ST_GeomFromText('POINT(-48.5493 -27.5936)', 4674), 'Mercado Público, Florianópolis', '09:00', '18:00', 6, 9),

       (ST_GeomFromText('POINT(-46.6308 -23.5475)', 4674), 'Mercado Municipal, São Paulo', '07:00', '16:00', 3, 12),
       (ST_GeomFromText('POINT(-46.6308 -23.5475)', 4674), 'Mercado Municipal, São Paulo', '07:00', '16:00', 4, 12),

       (ST_GeomFromText('POINT(-47.2186 -23.3696)', 4674), 'Feira de Hortifruti, Campinas', '08:00', '17:00', 1, 15),
       (ST_GeomFromText('POINT(-47.2186 -23.3696)', 4674), 'Feira de Hortifruti, Campinas', '08:00', '17:00', 2, 15),

       (ST_GeomFromText('POINT(-43.2072 -22.9035)', 4674), 'Feira Livre Ipanema, Rio de Janeiro', '06:00', '13:00', 5,
        17),
       (ST_GeomFromText('POINT(-43.2072 -22.9035)', 4674), 'Feira Livre Ipanema, Rio de Janeiro', '06:00', '13:00', 6,
        17);