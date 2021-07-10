pub mod arguments;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use anyhow::{anyhow, Result};
use arguments::{get_clap_app, Argv, Parameters};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::{Book, BookInsert};
use schema::*;
use std::{env, str::FromStr};

fn establish_connection() -> Result<MysqlConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    MysqlConnection::establish(&database_url)
        .map_err(|e| anyhow!("Error connecting to database: {}", e))
}

fn add_book(conn: &MysqlConnection, name: &str) -> Result<()> {
    diesel::insert_into(book_table::table)
        .values(BookInsert {
            title: name.to_owned(),
        })
        .execute(conn)?;
    Ok(())
}

fn get_book(conn: &MysqlConnection) -> Result<Vec<Book>> {
    Ok(book_table::table.load::<Book>(conn)?)
}

fn get_unread_book(conn: &MysqlConnection) -> Result<Vec<Book>> {
    use schema::book_table::dsl::done;
    Ok(book_table::table
        .filter(done.eq(false))
        .load::<Book>(conn)?)
}

fn mark_read(conn: &MysqlConnection, name: &str) -> Result<()> {
    use schema::book_table::dsl::{done, title};

    diesel::update(book_table::table.filter(title.eq(name)))
        .set(done.eq(true))
        .execute(conn)?;
    Ok(())
}

fn delete_book(conn: &MysqlConnection, name: &str) -> Result<()> {
    use schema::book_table::dsl::title;

    diesel::delete(book_table::table.filter(title.eq(name))).execute(conn)?;
    Ok(())
}

fn main() -> Result<()> {
    let connection = establish_connection()?;
    let matches = get_clap_app();
    let (command, submatches) = matches.subcommand();

    let verb = Parameters::from_str(command);

    match verb? {
        Parameters::add => {
            let book_name = submatches
                .unwrap()
                .value_of(Argv::book_name)
                .unwrap()
                .to_string();
            add_book(&connection, &book_name)?;
            println!("Book added");
        }
        Parameters::list => {
            let list_books;

            if submatches.unwrap().is_present(Argv::all) == true {
                list_books = get_book(&connection)?;
            } else {
                list_books = get_unread_book(&connection)?;
            }

            for i in list_books {
                println!("{}", i.title);
            }
        }
        Parameters::remove => {
            let book_name = submatches
                .unwrap()
                .value_of(Argv::book_name)
                .unwrap()
                .to_string();
            delete_book(&connection, &book_name)?;
            println!("Book deleted.")
        }
        Parameters::tick => {
            let book_name = submatches
                .unwrap()
                .value_of(Argv::book_name)
                .unwrap()
                .to_string();
            mark_read(&connection, &book_name)?;
            println!("Book marked as read.")
        }
    }

    Ok(())
}
