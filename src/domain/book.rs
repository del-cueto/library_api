use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub published_year: Option<i32>,
    pub created_at: String,
}

impl Book {
    pub fn new(title: String, author: String, published_year: Option<i32>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            author,
            published_year,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
