-- Add migration script here
-- Create Users Table
CREATE TABLE users(
	id uuid NOT NULL,
	PRIMARY KEY (id),
	email TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL,
	joined_at timestamptz NOT NULL
);
