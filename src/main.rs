#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

fn main() {
  dotenv().ok();

  let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
  let conn = PgConnection::establish(&db_url).unwrap();

  let book = models::NewBook {
    title: String::from("Gravity's Rainbow"),
    author: String::from("Thomas Pynchon"),
    published: true,
  };

  if models::Book::insert(book, &conn) {
    println!("success");
  } else {
    println!("failed");
  }
}
