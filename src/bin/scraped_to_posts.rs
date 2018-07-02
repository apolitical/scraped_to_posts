extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate r2d2;

extern crate scraped_to_posts;

use std::io::{self, prelude::*};
use scraped_to_posts::input::Article;
use dotenv::dotenv;
use diesel::{prelude::*, MysqlConnection};
use std::env;

fn get_db_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    dotenv().ok();

    let con = get_db_connection();

    let stdin = io::stdin();

    stdin.lock().lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .map(|line| serde_json::from_str::<Article>(&line))
        .filter(|article| article.is_ok())
        .map(|article| article.unwrap())
        .for_each(|article| {
            let inserted_post = article.create_post().insert(&con);
            if let Ok(post) = inserted_post {
                let meta = article.create_post_meta().into_iter()
                    .map(|m| m.attach_to_post(&post))
                    .map(|m| m.insert(&con));
                println!("{:?}", post);
                println!("{:?}", meta);
            }
        });
}
