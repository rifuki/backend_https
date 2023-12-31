-- Add up migration script here
CREATE TABLE credentials (
    username VARCHAR(25) NOT NULL,
    password VARCHAR(100) NOT NULL,
    refresh_token VARCHAR(255) NULL,
    max_age BIGINT NULL,
    PRIMARY KEY (username),
    FOREIGN KEY (username) REFERENCES user (username) ON DELETE CASCADE ON UPDATE CASCADE
);