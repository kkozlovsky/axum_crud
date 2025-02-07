CREATE TABLE public.users
(
    user_id SERIAL PRIMARY KEY,
    name    VARCHAR NOT NULL,
    age     INTEGER
);