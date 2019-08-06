extern crate diesel;

use super::schema::images;
use diesel::{prelude::*, PgConnection};
use std::collections::HashMap;

#[derive(Debug, AsChangeset, Queryable, Serialize)]
pub struct Image {
    pub id: i32,
    pub link: String,
    pub description: String,
}

#[derive(Insertable, FromForm)]
#[table_name = "images"]
pub struct NewImage {
    pub link: String,
    pub description: String,
}

impl NewImage {
    pub fn insert_self(self, conn: &PgConnection) -> usize {
        diesel::insert_into(images::table)
            .values(&self)
            .execute(conn)
            .expect("Error saving new post!")
    }
}

impl Image {
    // make new post
    pub fn insert(conn: &PgConnection, link: String, desc: String) -> usize {
        let new_name = NewImage {
            link: link,
            description: desc,
        };

        diesel::insert_into(images::table)
            .values(&new_name)
            .execute(conn)
            .expect("Error saving new post!")
    }
    pub fn all(conn: &PgConnection) -> HashMap<&str, Vec<Image>> {
        let mut results = HashMap::new();
        results.insert(
            "entries",
            images::table
                .order(images::id.desc())
                .load::<Image>(&*conn)
                .expect("Error! Could not get all entries."),
        );
        results
    }

    pub fn single(id: i32, conn: &PgConnection) -> Option<Image> {
        images::table.find(id).first::<Image>(conn).ok()
    }

    pub fn update(id: i32, image: Image, conn: &PgConnection) -> bool {
        diesel::update(images::table.find(id))
            .set(&image)
            .execute(conn)
            .is_ok()
    }
    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(images::table.find(id)).execute(conn).is_ok()
    }
}
