-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  post_type VARCHAR NOT NULL DEFAULT 'PORTFOLIO',
  description VARCHAR,
  image_url VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)
