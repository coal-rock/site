CREATE TABLE IF NOT EXISTS users (
	id integer primary key not null,
	password_hash text not null,
	name text not null unique
)

CREATE TABLE IF NOT EXISTS pages (
	title text not null unique,
	description text,
	content text not null,
	date text not null,
	serve_path text not null unique,
	template text not null
)


CREATE TABLE IF NOT EXISTS requests (
	url text not null,
	user_agent text,
	referer text,
	timestamp text not null
)
