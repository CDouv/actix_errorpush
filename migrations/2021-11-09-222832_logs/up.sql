CREATE TABLE logs
(
id SERIAL PRIMARY KEY,
username VARCHAR NOT NULL,
userid  VARCHAR NOT NULL,
environment  VARCHAR NOT NULL,
version  VARCHAR NOT NULL,
body VARCHAR NOT NULL
)

