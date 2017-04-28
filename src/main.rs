#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> PgConnection {
    // TODO(hyena): We ought to use a .toml config instead of dotenv.
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use schema::prices::dsl::*;

    let connection = establish_connection();
    let results = prices.filter(id.eq(124103))  // Foxflower
        .limit(5)
        .load::<Price>(&connection)
        .expect("Error loading foxflower prices.");
}
