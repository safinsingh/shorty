#[macro_use]
extern crate rocket;

mod api;
mod attribution;
mod auth;
mod db;

use self::{attribution::Attribution, db::ShortyDb};
use anyhow::{Context, Result};
use r2d2::Pool;
use redis::Client;
use rocket::{response::Redirect, State};
use std::env;

type ShortyState<'r> = State<'r, ShortyDb>;

#[get("/<name>")]
async fn link<'r>(
    state: ShortyState<'r>,
    name: Option<String>,
) -> Option<Redirect> {
    state
        .get_link(name.unwrap_or(String::from("root")))
        .await
        .map(|x| Redirect::temporary(x.url))
        .ok()
}

#[tokio::main]
async fn main() -> Result<()> {
    let url =
        env::var("DB_URL").context("DB_URL environment variable not set")?;
    let manager = Client::open(url).context("Error connecting to Redis")?;
    let pool = Pool::new(manager)?;

    rocket::ignite()
        .mount("/", routes![link, api::add_item, api::delete_item])
        .manage(ShortyDb::new(pool))
        .attach(Attribution)
        .launch()
        .await
        .map_err(Into::into)
}
