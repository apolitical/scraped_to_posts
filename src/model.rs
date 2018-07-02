use std::string::ToString;
use chrono::prelude::*;
use super::schema::{wp_posts, wp_posts::dsl::*, wp_postmeta, wp_postmeta::dsl::*};
use super::error::{Error, Result};
use diesel::{insert_into, MysqlConnection, RunQueryDsl, prelude::*};

const LPF_URL: &str = "nona_lpf_url";

#[derive(Identifiable, Insertable, Queryable, Serialize, Debug)]
#[table_name = "wp_posts"]
#[primary_key(ID)]
pub struct Post {
    #[column_name = "ID"]
    pub id: u64,
    pub post_author: u64,
    pub post_date: NaiveDateTime,
    pub post_date_gmt: NaiveDateTime,
    pub post_content: String,
    pub post_title: String,
    pub post_excerpt: String,
    pub post_status: String,
    pub comment_status: String,
    pub ping_status: String,
    pub post_password: String,
    pub post_name: String,
    pub to_ping: String,
    pub pinged: String,
    pub post_modified: NaiveDateTime,
    pub post_modified_gmt: NaiveDateTime,
    pub post_content_filtered: String,
    pub post_parent: u64,
    pub guid: String,
    pub menu_order: i32,
    pub post_type: String,
    pub post_mime_type: String,
    pub comment_count: i64,
}

impl Post {
    pub fn insert(self, connection: &MysqlConnection) -> Result<Post> {
        insert_into(wp_posts)
            .values(&self)
            .execute(connection)?;

        wp_posts
            .filter(post_title.eq(self.post_title))
            .filter(post_date.eq(self.post_date))
            .limit(1)
            .load::<Post>(connection)?
            .pop()
            .ok_or(Error::db_insert_error("wp_posts"))
    }
}

#[derive(Associations, Identifiable, Insertable, Queryable, Serialize, Debug)]
#[belongs_to(Post)]
#[primary_key(meta_id)]
#[table_name = "wp_postmeta"]
pub struct PostMeta {
    pub meta_id: u64,
    pub post_id: u64,
    pub meta_key: Option<String>,
    pub meta_value: Option<String>,
}

impl PostMeta {
    pub fn from_lpf_url<T>(url: T) -> PostMeta
        where T: AsRef<str> + ToString
    {
        PostMeta {
            meta_id: 0,
            post_id: 0,
            meta_key: Some(LPF_URL.to_string()),
            meta_value: Some(url.to_string()),
        }
    }

    pub fn attach_to_post(self, post: &Post) -> PostMeta {
        PostMeta { post_id: post.id.clone(), ..self }
    }

    pub fn insert(self, connection: &MysqlConnection) -> Result<PostMeta> {
        insert_into(wp_postmeta)
            .values(&self)
            .execute(connection)?;

        wp_postmeta
            .filter(post_id.eq(self.post_id))
            .filter(meta_key.eq(self.meta_key))
            .filter(meta_value.eq(self.meta_value))
            .limit(1)
            .load::<PostMeta>(connection)?
            .pop()
            .ok_or(Error::db_insert_error("wp_postmeta"))
    }
}
