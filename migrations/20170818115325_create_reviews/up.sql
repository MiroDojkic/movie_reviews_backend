-- Your SQL goes here
CREATE TABLE reviews (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  content VARCHAR NOT NULL,
  user_id INTEGER NOT NULL,
  movie_id INTEGER NOT NULL,
  FOREIGN KEY ("user_id") REFERENCES "public"."users"("id"),
  FOREIGN KEY ("movie_id") REFERENCES "public"."movies"("id")
);
