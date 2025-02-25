CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE,
    given_name TEXT NOT NULL,
    family_name TEXT NOT NULL,
    picture TEXT,
    provider VARCHAR(6) NOT NULL DEFAULT "google",
    provider_id TEXT,
    verified BOOLEAN NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
