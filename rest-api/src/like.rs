use actix_http::Response;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type Likes = Response<Like>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    pub id: String,
    pub created_at: DateTime<Utc>,
}
