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

    async fn get_by_id(&self, id: &str) -> Result<Option<Book>, anyhow::Error> {
        let book = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(book)
    }

    async fn create(&self, book: Book) -> Result<Book, anyhow::Error> {
        let query = r#"
            INSERT INTO books (id, title, author, published_year, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
        "#;

        sqlx::query(query)
            .bind(&book.id)
            .bind(&book.title)
            .bind(&book.author)
            .bind(book.published_year)
            .bind(&book.created_at)
            .execute(&self.pool)
            .await?;

        Ok(book)
    }
}
