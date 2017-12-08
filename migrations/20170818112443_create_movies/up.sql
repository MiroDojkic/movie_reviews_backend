-- Your SQL goes here
CREATE TABLE movies (
  id SERIAL PRIMARY KEY,
  name VARCHAR UNIQUE NOT NULL,
  director VARCHAR UNIQUE NOT NULL,
  year INTEGER NOT NULL,
  category_id INTEGER NOT NULL,
  FOREIGN KEY ("category_id") REFERENCES "public"."categories"("id")
);
