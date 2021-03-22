#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

mod schema;
mod models;
mod mpdctl;
mod radiofiles;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL in your .env");
    let _pg = PgConnection::establish(&database_url).unwrap();

    let mut mpc = mpdctl::mpd_connect().unwrap();
    match mpc.login("password") { // Auth with MPD server
        Ok(_client) => println!("Connected to MPD!"),
        Err(error) => panic!("Unable to connect to mpd: {:?}", error),
    };
    mpc.volume(100).unwrap();
    mpc.play().unwrap(); 

    println!("{}", radiofiles::upsert_db(radiofiles::get_radiofiles(&env::var("RADIOFILES_ROOT").expect("Please set RADIOFILES_URL in your .env"))).unwrap());
}

// Folder Structure: /system/game name (year)/song1.wav