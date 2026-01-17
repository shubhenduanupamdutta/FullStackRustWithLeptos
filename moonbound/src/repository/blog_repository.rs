use chrono::Local;
use leptos::prelude::*;

use crate::model::blog_post::Post;

#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};
#[cfg(feature = "ssr")]
use leptos_actix::extract;



#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    text: String,
) -> Result<String, ServerFnError> {
    Ok(String::from("placeholder"))
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    Ok(Post {
        id: "1".to_string(),
        dt: Local::now().naive_local(),
        title: "Ocean View".to_string(),
        image_url: "https://bit.ly/3t0bA61".to_string(),
        text: "I spent some time at the beach today and it was wonderful!".to_string(),
    })
}

#[server(GetPreviews, "/api")]
pub async fn get_previews() -> Result<Vec<Post>, ServerFnError> {
    Ok(vec![
        Post {
            id: "1".to_string(),
            dt: Local::now().naive_local(),
            title: "Ocean View".to_string(),
            image_url: "https://bit.ly/3t0bA61".to_string(),
            text: "I spent some time at the beach today and it was wonderful!".to_string(),
        },
        Post {
            id: "2".to_string(),
            dt: Local::now().naive_local(),
            title: "Desert".to_string(),
            image_url: "https://bit.ly/3t8HGMG".to_string(),
            text: "The desert is vast and mysterious and I got lost in its beauty.".to_string(),
        },
        Post {
            id: "3".to_string(),
            dt: Local::now().naive_local(),
            title: "Garden".to_string(),
            image_url: "https://bit.ly/3RfUxop".to_string(),
            text: "Walking through the garden filled me with peace and joy.".to_string(),
        },
        Post {
            id: "4".to_string(),
            dt: Local::now().naive_local(),
            title: "Andromeda".to_string(),
            image_url: "https://bit.ly/47PKLQQ".to_string(),
            text: "Gazing at the Andromeda galaxy made me feel so small yet connected.".to_string(),
        },
    ])
}
