#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use dotenv::dotenv;
use photoblog::{
    models::{Image, NewImage},
    ImageDb,
};
use rocket::config::{Config, Environment, Value};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::collections::HashMap;
use std::env::var;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::SocketAddr;

#[get("/", rank = 2)]
fn home(conn: ImageDb, remote_address: SocketAddr) -> Template {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    file.write_all(format!("{}\n", remote_address).as_bytes());

    Template::render("index", Image::all(&conn))
}

#[get("/upload")]
fn upload() -> Template {
    let mut c = HashMap::new();
    c.insert("empty", "empty");
    Template::render("upload", c)
}

#[post("/upload", data = "<mainform>")]
fn insert(conn: ImageDb, mainform: Form<NewImage>) -> Redirect {
    mainform.into_inner().insert_self(&conn);
    Redirect::to(uri!(home))
}

#[get("/edit/<id>")]
fn get_edit_page(conn: ImageDb, id: i32) -> Template {
    let mut default = HashMap::new();
    default.insert("link", "NOTFOUND");
    default.insert("description", "NOTFOUND");

    println!("{:?}", Image::single(id, &conn));
    match Image::single(id, &conn) {
        Some(image) => Template::render("edit", image),
        None => Template::render("edit", default),
    }
}

#[post("/edit/<id>", data = "<mainform>")]
fn post_edit_page(conn: ImageDb, id: i32, mainform: Form<NewImage>) -> Redirect {
    let im = mainform.into_inner();
    let new_content = Image {
        id: id,
        link: im.link,
        description: im.description,
    };

    Image::update(id, new_content, &conn);
    Redirect::to(uri!(home))
}

#[get("/?<delete>")]
fn delete(conn: ImageDb, delete: i32) -> Template {
    // delete based on id
    Image::delete(delete, &*conn);
    Template::render("index", Image::all(&*conn))
}

fn main() {
    dotenv().ok();
    rocket::custom(make_config())
        .attach(Template::fairing())
        .attach(ImageDb::fairing())
        .mount("/", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![home, upload, insert, delete, get_edit_page, post_edit_page],
        )
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
