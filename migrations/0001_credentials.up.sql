-- Add up migration script here
CREATE TABLE credentials (
    username VARCHAR(25) NOT NULL,
    password VARCHAR(100) NOT NULL,
    PRIMARY KEY (username)
);