#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

pub mod schema;
pub mod models;

use rocket_contrib::databases::diesel::PgConnection;

// database stuff
#[database("image_db")]
pub struct ImageDb(PgConnection);
