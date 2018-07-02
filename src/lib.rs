#[macro_use]
extern crate diesel;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

extern crate chrono;
extern crate serde;

pub mod error;
pub mod input;
pub mod model;
mod schema;

///// Consumes an input object and returns a Wordpress Post
/////
//fn input_to_model(input: Article) -> Post {
//    Post {
//        id: u64,
//        post_author: u64,
//        post_date: NaiveDateTime,
//        post_date_gmt: NaiveDateTime,
//        post_content: String,
//        post_title: String,
//        post_excerpt: String,
//        post_status: String,
//        comment_status: String,
//        ping_status: String,
//        post_password: String,
//        post_name: String,
//        to_ping: String,
//        pinged: String,
//        post_modified: NaiveDateTime,
//        post_modified_gmt: NaiveDateTime,
//        post_content_filtered: String,
//        post_parent: u64,
//        guid: String,
//        menu_order: i32,
//        post_type: String,
//        post_mime_type: String,
//        comment_count: i64,
//    }
//}
