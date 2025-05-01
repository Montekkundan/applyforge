-- SQL schema for ApplyForge

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS jobs (
    id SERIAL PRIMARY KEY,
    company TEXT NOT NULL,
    position TEXT NOT NULL,
    status TEXT NOT NULL,
    date TEXT NOT NULL,
    username TEXT NOT NULL
);
