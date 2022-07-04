pub mod users;
pub mod games;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel_migrations::embed_migrations;

use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("SCHEDULE_DATABASE_URL")
        .expect("SCHEDULE_DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

embed_migrations!();
pub fn run_migrations() {
    println!("Running DB migrations");
    let conn = establish_connection();
    match embedded_migrations::run(&conn) {
        Ok(_) => println!("Migrations run successfully"),
        Err(e) => println!("Error running migrations: {}", e)
    }
}
