use chrono::{DateTime, Utc, TimeZone};
use chrono::format::ParseError;
use super::model::{Post, PostMeta};

const DRAFT : &str = "draft";
const CLOSED : &str = "closed";
const BRIEFING : &str = "briefing";
const POST_AUTHOR: u64 = 1320;

#[derive(Deserialize, Debug)]
pub struct Author {
    #[serde(rename = "@type")]
    pub author_type: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Organisation {
    #[serde(rename = "@type")]
    pub organisation_type: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    name: String,
    author: Vec<Author>,
    source_organization: Organisation,
    #[serde(with = "optional_stupid_date_format")]
    date_published: Option<DateTime<Utc>>,
    summary: String,
    url: String,
    #[serde(with = "stupid_date_format")]
    scraped_at: DateTime<Utc>,
    spider_name: String,
    categories: String,
}

impl Article {
    fn name_to_name(&self) -> String {
        self.name.clone().replace(" ", "-").to_lowercase()
    }

    pub fn create_post(&self) -> Post {
        let post_name = self.name_to_name();
        let publish_date = self.date_published.unwrap_or(self.scraped_at).naive_utc();
        let now = Utc::now().naive_utc();
        Post {
            id: 0,
            post_author: POST_AUTHOR,
            post_date: publish_date,
            post_date_gmt: publish_date,
            post_content: self.summary.clone(),
            post_title: self.name.clone(),
            post_excerpt: self.summary.clone(),
            post_status: DRAFT.to_string(),
            comment_status: CLOSED.to_string(),
            ping_status: CLOSED.to_string(),
            post_password: "".to_string(),
            post_name,
            to_ping: "".to_string(),
            pinged: "".to_string(),
            post_modified: now,
            post_modified_gmt: now,
            post_content_filtered: "".to_string(),
            post_parent: 0,
            guid: "".to_string(),
            menu_order: 0,
            post_type: BRIEFING.to_string(),
            post_mime_type: "".to_string(),
            comment_count: 0,
        }
    }

    pub fn create_post_meta(&self) -> Vec<PostMeta> {
        let mut post_meta = Vec::with_capacity(1);
        post_meta.push(PostMeta::from_lpf_url(self.url.clone()));
        post_meta
    }
}

/// This function takes a date string and tries to guess at how to turn it into a DateTime<UTC>
fn ham_date(s: String) -> Result<DateTime<Utc>, ParseError> {
    if let Ok(dt) = Utc.datetime_from_str(&s, "%Y-%m-%d %H:%M:%S") {
        Ok(dt)
    } else {
        Utc.datetime_from_str(&s, "%Y-%m-%dT%H:%M:%S.%f")
    }
}

/// Uses ham_date to guess at the result
mod stupid_date_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        super::ham_date(s).map_err(serde::de::Error::custom)
    }
}

/// Uses ham_date to guess at the result
mod optional_stupid_date_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).ok();
        if s.is_none() {
            Ok(None)
        } else {
            Ok(Some(super::ham_date(s.unwrap()).map_err(serde::de::Error::custom)?))
        }
    }
}
