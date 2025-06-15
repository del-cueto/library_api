use crate::domain::book::Book;
use async_trait::async_trait;

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Book>, anyhow::Error>;
}
