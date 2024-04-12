use std::error::Error;
use std::sync::Arc;

pub type ID = String;

pub trait Entity {
    fn get_id(&self) -> ID;
}

pub trait Repository<T: Entity> {
    fn insert(&self, t: T) -> Result<(), RepositoryError>;
    fn update(&self, t: T) -> Result<(), RepositoryError>;
    fn get(&self, id: &String) -> Result<T, RepositoryError>;
    fn delete(&self, id: &String) -> Result<(), RepositoryError>;
}

pub type OwnedRepository<T> = Box<dyn Repository<T> + Send + Sync>;
pub type SharedRepository<T> = Arc<dyn Repository<T> + Send + Sync>;

#[derive(Debug)]
pub enum RepositoryError {
    Internal(Box<dyn Error>),
    AlreadyExists(ID),
    NotFound(ID),
}
