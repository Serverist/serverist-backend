-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users
(
    id BINARY(16) PRIMARY KEY,
    created_at DATETIME NOT NULL,
    account_id VARCHAR(15) NOT NULL,
    name VARCHAR(31) NOT NULL,
    description VARCHAR(255) NOT NULL,
    icon_id BINARY(16) NOT NULL
);