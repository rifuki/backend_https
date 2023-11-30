-- Add up migration script here
CREATE TABLE user (
    id_user INT AUTO_INCREMENT,
    full_name VARCHAR(100),
    email VARCHAR(50),
    phone_number VARCHAR(14),
    username VARCHAR(25) NOT NULL,
    password VARCHAR(100) NOT NULL,
    PRIMARY KEY (id_user),
    UNIQUE (email),
    UNIQUE (phone_number),
    UNIQUE (username)
);