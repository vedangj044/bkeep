pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::{Book, BookInsert};
use schema::*;
use std::env;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url).expect("Error connecting to database")
}

fn main() {
    let connection = establish_connection();

    let obj = BookInsert {
        title: "Alchemist".to_owned(),
    };

    diesel::insert_into(book_table::table)
        .values(&obj)
        .execute(&connection)
        .expect("Can't insert into database");

    let me = book_table::table
        .load::<Book>(&connection)
        .expect("Error loading error");

    for i in me {
        println!("{:?}", i);
    }
}
