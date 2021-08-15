use crate::response::Response;
use actix_web::web::Path;
use actix_web::{get, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const APPLICATION_JSON: &'static str = "application/json";

pub type Likes = Response<Like>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Like {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }
}

#[get("/tweets/{id}/likes")]
pub async fn list(path: Path<(String,)>) -> HttpResponse {
    println!("{}", path.0 .0);

    let mut likes = Likes::new();

    likes.results = vec![Like::new()];

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}
