use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

pub type ID = String;

pub trait Entity {
    fn get_id(&self) -> ID;
}

#[async_trait]
pub trait Repository<T: Entity + Send + Sync> {
    async fn insert(&self, t: T) -> Result<(), RepositoryError>;
    async fn update(&self, t: T) -> Result<(), RepositoryError>;
    async fn get(&self, id: &String) -> Result<T, RepositoryError>;
    async fn delete(&self, id: &String) -> Result<(), RepositoryError>;
}

pub type OwnedRepository<T> = Box<dyn Repository<T> + Send + Sync>;
pub type SharedRepository<T> = Arc<dyn Repository<T> + Send + Sync>;

#[derive(Debug)]
pub enum RepositoryError {
    Internal(Box<dyn Error>),
    AlreadyExists(ID),
    NotFound(ID),
}
