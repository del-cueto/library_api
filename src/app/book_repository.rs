use crate::domain::book::Book;
use async_trait::async_trait;

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Book>, anyhow::Error>;
    async fn get_by_id(&self, id: &str) -> Result<Option<Book>, anyhow::Error>;
    async fn create(&self, book: Book) -> Result<String, anyhow::Error>;
}