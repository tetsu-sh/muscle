-- Your SQL goes here

CREATE TABLE accounts(
    id VARCHAR(100) PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE trains (
    id VARCHAR(100) PRIMARY KEY,
    account_id VARCHAR(100) NOT NULL,
    name VARCHAR(100) NOT NULL,
    volume INTEGER NOT NULL, 
    rep INTEGER NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);

CREATE TABLE body_parts(
    id VARCHAR(100) PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE muscles(
    id VARCHAR(100) PRIMARY KEY,
    body_part_id VARCHAR(100) NOT NULL,
    name VARCHAR(100) NOT NULL,
    size VARCHAR(100) NOT NULL,
    FOREIGN KEY (body_part_id) REFERENCES body_parts(id) 
);

CREATE TABLE train_muscle(
    train_id VARCHAR(100) NOT NULL,
    muscle_id VARCHAR(100) NOT NULL,
    PRIMARY KEY(train_id,muscle_id),
    FOREIGN KEY (train_id) REFERENCES trains(id),
    FOREIGN KEY (muscle_id) REFERENCES muscles(id)
);

CREATE TABLE muscle_damages(
    id VARCHAR(100) PRIMARY KEY,
    account_id VARCHAR(100) NOT NULL,
    muscle_id VARCHAR(100) NOT NULL,
    value INTEGER  NOT NULL DEFAULT 0,
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (muscle_id) REFERENCES muscles(id)
);

CREATE TABLE body_conditions(
    id VARCHAR(100) PRIMARY KEY,
    account_id VARCHAR(100) NOT NULL,
    body_part_id VARCHAR(100) NOT NULL,
    eccumulative_value INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (body_part_id) REFERENCES body_parts(id)
);

CREATE TABLE body_profile(
    id VARCHAR(100) PRIMARY KEY,
    account_id VARCHAR(100) NOT NULL,
    weight INTEGER DEFAULT 0,
    height INTEGER DEFAULT 0,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
    );

CREATE TABLE trainees(
    id VARCHAR(100) PRIMARY KEY,
    account_id VARCHAR(100) NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);