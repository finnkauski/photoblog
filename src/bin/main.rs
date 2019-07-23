#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use dotenv::dotenv;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::config::{Config, Environment, Value};
use rocket_contrib::{templates::Template, serve::StaticFiles};
use std::env::var;
use std::collections::HashMap;
use photoblog::{ImageDb, models::{Image, NewImage}};

#[get("/", rank = 2)]
fn home(conn: ImageDb) -> Template {
    Template::render("index", Image::all(&conn))
}

#[get("/upload")]
fn upload() -> Template {
    let mut c = HashMap::new();
    c.insert("empty", "empty");
    Template::render("upload", c)
}

#[get("/?<delete>" )]
fn delete(conn: ImageDb, delete: i32) -> Template {
    // delete based on id
    Image::delete(delete, &*conn);
    println!("trying to delete");
    Template::render("index", Image::all(&*conn))
}

#[post("/upload", data = "<mainform>")]
fn insert(conn: ImageDb, mainform: Form<NewImage>) -> Redirect {
    mainform.into_inner().insert_self(&conn);
    Redirect::to(uri!(home))
}

fn main() {
    dotenv().ok();
    rocket::custom(make_config())
        .attach(Template::fairing())
        .attach(ImageDb::fairing())
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![home, upload, insert, delete])
        .launch();
}

fn make_config() -> Config {
    let port: u16 = var("PORT").unwrap().parse().unwrap();
    println!("Found port {}", port);
    // create the dictionaries for the values of database information
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(var("DATABASE_URL").unwrap()));
    databases.insert("image_db", Value::from(database_config));

    let config = Config::build(Environment::Production)
        .port(port)
        .extra("template_dir", "static") // add static template directory
        .extra("databases", databases) // add the databases to the config
        .finalize()
        .unwrap();
    config
}

