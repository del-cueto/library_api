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

#[cfg(test)]
mod tests {
    use super::Book;
    use chrono::{Utc, Duration, DateTime};
    use uuid::Uuid;

    #[test]
    fn new_book_generates_valid_and_unique_ids() {
        let b1 = Book::new("Title".into(), "Author".into(), Some(2023));
        let b2 = Book::new("Title".into(), "Author".into(), Some(2023));

        assert!(Uuid::parse_str(&b1.id).is_ok());
        assert!(Uuid::parse_str(&b2.id).is_ok());
        assert_ne!(b1.id, b2.id);
    }

    #[test]
    fn new_book_timestamp_is_recent() {
        let before = Utc::now() - Duration::seconds(1);
        let book = Book::new("Foo".into(), "Bar".into(), None);
        let after = Utc::now() + Duration::seconds(1);

        // Parse el campo String a DateTime<Utc>
        let ts: DateTime<Utc> = DateTime::parse_from_rfc3339(&book.created_at)
            .expect("created_at debe ser RFC 3339")
            .with_timezone(&Utc);

        assert!(
            ts >= before && ts <= after,
            "created_at ({}) no está en el rango [{}, {}]",
            ts, before, after
        );
    }
}
