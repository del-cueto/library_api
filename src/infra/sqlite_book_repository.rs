use crate::{app::book_repository::BookRepository, domain::book::Book};
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqliteBookRepository {
    pub pool: SqlitePool,
}

#[async_trait]
impl BookRepository for SqliteBookRepository {
    async fn get_all(&self) -> Result<Vec<Book>, anyhow::Error> {
        let books = sqlx::query_as::<_, Book>("SELECT * FROM books")
            .fetch_all(&self.pool)
            .await?;
        Ok(books)
    }
}
