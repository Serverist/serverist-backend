-- Your SQL goes here

CREATE TABLE users
(
    id BINARY(16) PRIMARY KEY,
    account_id VARCHAR(15) NOT NULL,
    name VARCHAR(31) NOT NULL,
    description VARCHAR(255) NOT NULL,
    icon_id BINARY(16) NOT NULL
);