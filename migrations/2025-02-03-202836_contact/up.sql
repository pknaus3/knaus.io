-- Your SQL goes here
CREATE TABLE contacts (
   id SERIAL PRIMARY KEY,
   name VARCHAR NOT NULL,
   email VARCHAR NOT NULL,
   subject varchar NOT NULL,
   body TEXT NOT NULL,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);