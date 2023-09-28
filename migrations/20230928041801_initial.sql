-- Add migration script here
CREATE TABLE twat (
  id INTEGER PRIMARY KEY,
  date TEXT NOT NULL,
  user_id TEXT NOT NULL,
  url TEXT NOT NULL
);
