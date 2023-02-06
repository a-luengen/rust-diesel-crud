use crate::db::database::establish_connection;

use self::db::models::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
mod db;

pub fn fetch_posts(con: &mut SqliteConnection) {
    use self::db::schema::posts::dsl::*;

    let result = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(con)
        .expect("Error loading posts");

    println!("Display {}", result.len());

    for post in result {
        println!("{}", post.title);
        println!("{}", post.body)
    }
}

pub fn create_post(
    con: &mut SqliteConnection,
    title: &str,
    body: &str,
    creation_date: &NaiveDateTime,
) {
    use crate::db::schema::posts;

    let new_post = NewPost {
        title,
        body,
        creation_date,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(con)
        .expect("Error saving new post");
}

fn main() {
    let con = &mut establish_connection();

    create_post(
        con,
        "Some title",
        "Here is body",
        &DateTime::naive_utc(&Utc::now()),
    );

    fetch_posts(con);
}
