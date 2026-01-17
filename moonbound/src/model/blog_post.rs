use serde::{Deserialize, Serialize};

#[cfg(all(not(feature = "ssr"), feature = "hydrate"))]
use chrono::Local;
#[cfg(all(not(feature = "ssr"), feature = "hydrate"))]
use chrono::NaiveDateTime;
#[cfg(feature = "ssr")]
use sqlx::types::chrono::Local;
#[cfg(feature = "ssr")]
use sqlx::types::chrono::NaiveDateTime;
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(Serialize, Deserialize, Debug, Clone, FromRow))]
#[cfg_attr(
    all(not(feature = "ssr"), feature = "hydrate"),
    derive(Serialize, Deserialize, Debug, Clone)
)]
pub struct Post {
    pub id: String,
    pub dt: NaiveDateTime,
    pub image_url: String,
    pub title: String,
    pub text: String,
}

impl Post {
    pub fn new_empty() -> Post {
        Post {
            id: "".to_string(),
            dt: Local::now().naive_local(),
            image_url: "".to_string(),
            title: "".to_string(),
            text: "".to_string(),
        }
    }
}
