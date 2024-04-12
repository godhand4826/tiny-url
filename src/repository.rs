use crate::core::RepositoryError;
use crate::core::{Entity, Repository, ID};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub struct HashMapRepository<T: Entity> {
    hash_map: Arc<Mutex<HashMap<ID, T>>>,
}

impl<T: Entity + Send + Sync> HashMapRepository<T> {
    pub fn new() -> HashMapRepository<T> {
        HashMapRepository::<T> {
            hash_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn size(&self) -> usize {
        self.hash_map.lock().unwrap().len()
    }
}

impl<T: Entity + Clone> Repository<T> for HashMapRepository<T> {
    fn insert(&self, t: T) -> Result<(), RepositoryError> {
        let id = t.get_id();
        let mut map = self.hash_map.lock().unwrap();

        if map.contains_key(&id) {
            return Result::Err(RepositoryError::AlreadyExists(id));
        } else {
            map.insert(id, t);
        }

        if map.len() % 10000 == 0 {
            println!("HashMapRepository size: {}", map.len());
        }

        Result::Ok(())
    }

    fn update(&self, t: T) -> Result<(), RepositoryError> {
        let id = t.get_id();

        self.hash_map
            .lock()
            .unwrap()
            .get_mut(&id)
            .map(|x| *x = t)
            .ok_or(RepositoryError::NotFound(id))
    }

    fn get(&self, id: &String) -> Result<T, RepositoryError> {
        self.hash_map
            .lock()
            .unwrap()
            .get(id)
            .map(|v| v.clone())
            .ok_or(RepositoryError::NotFound(id.clone()))
    }

    fn delete(&self, id: &String) -> Result<(), RepositoryError> {
        match self.hash_map.lock().unwrap().remove(id) {
            Some(_) => Result::Ok(()),
            None => Result::Err(RepositoryError::NotFound(id.clone())),
        }
    }
}
