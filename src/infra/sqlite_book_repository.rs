use crate::{
    app::book_repository::BookRepository,
    domain::book::Book,
};
use async_trait::async_trait;
use sqlx::SqlitePool;
use anyhow::Error;

pub struct SqliteBookRepository {
    pub pool: SqlitePool,
}

#[async_trait]
impl BookRepository for SqliteBookRepository {
    async fn get_all(&self) -> Result<Vec<Book>, Error> {
        let books = sqlx::query_as::<_, Book>("SELECT * FROM books")
            .fetch_all(&self.pool)
            .await?;
        Ok(books)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Book>, Error> {
        let book = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(book)
    }

    async fn create(&self, book: Book) -> Result<Book, Error> {
        sqlx::query(
            r#"
            INSERT INTO books (id, title, author, published_year, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
            .bind(&book.id)
            .bind(&book.title)
            .bind(&book.author)
            .bind(book.published_year)
            .bind(&book.created_at)
            .execute(&self.pool)
            .await?;
        Ok(book)
    }

    async fn update(&self, book: Book) -> Result<Book, Error> {
        sqlx::query(
            r#"
            UPDATE books
               SET title = ?1,
                   author = ?2,
                   published_year = ?3
             WHERE id = ?4
            "#,
        )
            .bind(&book.title)
            .bind(&book.author)
            .bind(book.published_year)
            .bind(&book.id)
            .execute(&self.pool)
            .await?;
        Ok(book)
    }

    async fn delete(&self, id: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM books WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn search(
        &self,
        title: Option<&str>,
        author: Option<&str>,
    ) -> Result<Vec<Book>, Error> {
        let mut sql = String::from("SELECT * FROM books");
        let mut binds = Vec::new();

        if title.is_some() || author.is_some() {
            sql.push_str(" WHERE ");
            let mut clauses = Vec::new();

            if let Some(t) = title {
                clauses.push("title LIKE '%' || ? || '%'");
                binds.push(t);
            }
            if let Some(a) = author {
                clauses.push("author LIKE '%' || ? || '%'");
                binds.push(a);
            }

            sql.push_str(&clauses.join(" AND "));
        }

        let mut query = sqlx::query_as::<_, Book>(&sql);
        for &b in &binds {
            query = query.bind(b);
        }

        let books = query.fetch_all(&self.pool).await?;
        Ok(books)
    }
}
