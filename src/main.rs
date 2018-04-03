#![feature(plugin)]
#![plugin(rocket_codegen)]

// Rocket
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

// Diesel
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod connection;
mod services;

use rocket_contrib::{Json, Value};
use models::request::post::{Post};
use services::post_service;

#[post("/portfolio", format = "application/json", data = "<post>")]
fn create_portfolio(post: Json<Post>) -> Json {
    println!("{:?}", post);
    post_service::create_portfolio(post)
}

// to-do: Check ID before getting into the next step (panicked once).
#[put("/portfolio", format = "application/json", data = "<post>")]
fn update_portfolio(post: Json<Post>) -> Json {
    println!("{:?}", post);
    post_service::update_portfolio(post)
}

#[delete("/portfolio/<id>")]
fn remove_portfolio(id: i32) -> Json {
    println!("{:?}", id);
    post_service::remove_portfolio(id)
}

fn main() {
    rocket::ignite().mount("/", routes![create_portfolio, update_portfolio, remove_portfolio]).launch();
}
