CREATE TABLE users (
   id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
   email TEXT UNIQUE NOT NULL,
   given_name TEXT NOT NULL,
   family_name TEXT NOT NULL,
   picture TEXT,
   verified BOOLEAN NOT NULL
);
