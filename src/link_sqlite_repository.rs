use crate::core::Repository;
use crate::core::RepositoryError;
use crate::link::Link;
use async_trait::async_trait;
use r2d2::ManageConnection;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

pub struct LinkSqliteRepository<T: ManageConnection> {
    pool: Pool<T>,
}

impl<T: ManageConnection> LinkSqliteRepository<T> {
    pub fn new(pool: Pool<T>) -> LinkSqliteRepository<T> {
        LinkSqliteRepository { pool }
    }
}

#[async_trait]
impl Repository<Link> for LinkSqliteRepository<SqliteConnectionManager> {
    async fn insert(&self, link: Link) -> Result<(), RepositoryError> {
        self.pool
            .clone()
            .get()
            .unwrap()
            .prepare("INSERT INTO links (id, url, created_at) VALUES (?1, ?2, ?3)")
            .map_err(|err| RepositoryError::Internal(Box::new(err)))?
            .execute(params![&link.id, &link.url, &link.created_at])
            .map_err(|err| RepositoryError::Internal(Box::new(err)))
            .map(|_| ())
    }

    async fn update(&self, link: Link) -> Result<(), RepositoryError> {
        self.pool
            .clone()
            .get()
            .map_err(|err| RepositoryError::Internal(Box::new(err)))?
            .prepare("UPDATE links (id, url, created_at) VALUES (?1, ?2, ?3) WHERE id = ?1")
            .map_err(|err| RepositoryError::Internal(Box::new(err)))?
            .execute(params![&link.id, &link.url, &link.created_at])
            .map_err(|err| RepositoryError::Internal(Box::new(err)))
            .map(|_| ())
    }

    async fn get(&self, id: &String) -> Result<Link, RepositoryError> {
        let links: Vec<Link> = self
            .pool
            .clone()
            .get()
            .map_err(|err| RepositoryError::Internal(Box::new(err)))?
            .prepare("SELECT id, url, created_at FROM links WHERE id = ?1")
            .map_err(|err| RepositoryError::Internal(Box::new(err)))?
            .query_map(params![id], |row| -> Result<Link, rusqlite::Error> {
                Ok(Link {
                    id: row.get(0)?,
                    url: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })
            .and_then(Iterator::collect)
            .map_err(|err| RepositoryError::Internal(Box::new(err)))?;

        match links.first() {
            Some(link) => Ok(link.clone()),
            None => Err(RepositoryError::NotFound(id.to_string())),
        }
    }

    async fn delete(&self, id: &String) -> Result<(), RepositoryError> {
        self.pool
            .clone()
            .get()
            .map_err(|err| RepositoryError::Internal(Box::new(err)))?
            .prepare("DELETE FROM links WHERE id = ?1")
            .map_err(|err| RepositoryError::Internal(Box::new(err)))?
            .execute(params![id])
            .map_err(|err| RepositoryError::Internal(Box::new(err)))
            .map(|_| ())
    }
}
