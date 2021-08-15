use actix_web::{get, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{like::Like, response::Response};

pub type Tweets = Response<Tweet>;

const APPLICATION_JSON: &'static str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweet {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message.to_string())),
            None => None,
        }
    }
}

#[get("/tweets")]
pub async fn list() -> HttpResponse {
    let mut tweets = Tweets::new();

    tweets.results = vec![Tweet::new(String::from("google.com"))];

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}
