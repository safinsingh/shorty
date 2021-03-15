#![feature(proc_macro_hygiene, decl_macro)]

// 🏡 Local module imports
mod api;
mod attribution;
mod auth;
mod db;

use attribution::Attribution;
use db::ShortyDb;

// 👽 External create imports
#[macro_use]
extern crate rocket;

use redis::Client;
use rocket::{response::Redirect, State};
use std::{env, sync::RwLock};

pub struct ShortyState {
    db: RwLock<ShortyDb>,
}

#[get("/<name>")]
fn link(state: State<ShortyState>, name: String) -> Option<Redirect> {
    state
        .db
        .write()
        .unwrap()
        .get_link(&name)
        .map(|x| Redirect::temporary(x.url))
        .ok()
}

#[get("/")]
fn index() -> Option<Redirect> {
    env::var("ROOT_URL").map(|x| Redirect::temporary(x)).ok()
}

#[catch(404)]
fn not_found() -> String {
    String::from("404 not found")
}

fn main() {
    // Make sure certain environment variables are set
    env::var("DB_URL").expect("DB_URL environment variable not set");

    let redis_client = Client::open(env::var("DB_URL").expect("Missing DB_URL env variable."))
        .expect("Error connecting to Redis");
    let db = ShortyDb::new(redis_client);

    rocket::ignite()
        .mount("/", routes![index, link, api::add_item, api::delete_item])
        .register(catchers![not_found])
        .manage(ShortyState {
            db: RwLock::new(db),
        })
        .attach(Attribution)
        .launch();
}
