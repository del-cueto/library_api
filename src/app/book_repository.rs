use crate::domain::book::Book;
use async_trait::async_trait;

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Book>, anyhow::Error>;
    async fn get_by_id(&self, id: &str) -> Result<Option<Book>, anyhow::Error>;
    async fn create(&self, book: Book) -> Result<Book, anyhow::Error>;
    async fn update(&self, book: Book) -> Result<Book, anyhow::Error>;
    async fn delete(&self, id: &str) -> Result<(), anyhow::Error>;
    async fn search(
        &self,
        title: Option<&str>,
        author: Option<&str>,
    ) -> Result<Vec<Book>, anyhow::Error>;
}