#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate postgres;
extern crate chrono;

use chrono::NaiveDateTime;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use postgres::{NoTls, Client};
use std::env;


#[derive(FromForm)]
struct UserInput {
    pullups: i32,
    datetime: String
}


#[get("/")]
fn index() -> Template {
    let context = json!({});
    Template::render("index", context)
}

#[post("/add", data = "<user_input>")]
fn add_pullups(user_input: Form<UserInput>) -> Redirect {
    println!("Pullups: {}", user_input.pullups);
    println!("Date: {}", user_input.datetime);
    query_db(user_input.pullups, user_input.datetime.clone());
    Redirect::found("/added")
}

#[get("/added")]
fn added() -> Template {
    let context = json!({});
    Template::render("added", context)
}

fn query_db(pullups: i32, date: String) {
    let parsed_datetime = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap();
    let pg_conn = format!(
        "host={} user={} password={} dbname={}",
        env::var("DB_HOST").unwrap(),
        env::var("DB_USER").unwrap(),
        env::var("DB_PASS").unwrap(),
        env::var("DB_NAME").unwrap(),
    );
    let mut pg_client = Client::connect(&pg_conn, NoTls).unwrap();
    let query_res = pg_client.query_one(
        "INSERT INTO pullups (pullups, date) VALUES ($1, $2) RETURNING session_id",
        &[&pullups, &parsed_datetime]
    );
    match query_res {
        Ok(row) => {
            let session_id: i32 = row.get("session_id");
            println!("Created new pullup session: {}", session_id);
        },
        Err(err) => {
            println!("There was an error storing to DB! {}", err);
        }
    };
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index, add_pullups, added
        ])
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();
}
