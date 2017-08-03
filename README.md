# :clapper: Movie reviews backend

Checking out Rust ecosystem for building RESTful API.


# Dependencies
- [PostgreSQL](https://www.postgresql.org/)
- [Rust](https://www.rust-lang.org/)


# Run project 
1) Clone repository
2) Create database
3) Set `DATABASE_URL` env variable in `.env` file accordingly:  
`echo DATABASE_URL=postgres://<user>:<password>@localhost/<database_name> > .env`
4) Run `cargo run`

# Deployment
CI is set using Heroku, last version at:  
:point_right: [movie-reviews-api.herokuapp.com](https://movie-reviews-api.herokuapp.com)
