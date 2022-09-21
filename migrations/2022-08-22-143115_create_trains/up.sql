-- Your SQL goes here
CREATE TABLE trains (
    id VARCHAR(100) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    volume INTEGER NOT NULL, 
    rep INTEGER NOT NULL, 
    `set` INTEGER NOT NULL 
);