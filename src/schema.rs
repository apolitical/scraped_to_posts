table! {
    wp_posts (ID) {
        ID -> Unsigned<Bigint>,
        post_author -> Unsigned<Bigint>,
        post_date -> Datetime,
        post_date_gmt -> Datetime,
        post_content -> Longtext,
        post_title -> Text,
        post_excerpt -> Text,
        post_status -> Varchar,
        comment_status -> Varchar,
        ping_status -> Varchar,
        post_password -> Varchar,
        post_name -> Varchar,
        to_ping -> Text,
        pinged -> Text,
        post_modified -> Datetime,
        post_modified_gmt -> Datetime,
        post_content_filtered -> Longtext,
        post_parent -> Unsigned<Bigint>,
        guid -> Varchar,
        menu_order -> Integer,
        post_type -> Varchar,
        post_mime_type -> Varchar,
        comment_count -> Bigint,
    }
}

table! {
    wp_postmeta (meta_id) {
        meta_id -> Unsigned<Bigint>,
        post_id -> Unsigned<Bigint>,
        meta_key -> Nullable<Varchar>,
        meta_value -> Nullable<Longtext>,
    }
}

allow_tables_to_appear_in_same_query!(wp_posts, wp_postmeta);
