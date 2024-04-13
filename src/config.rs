use crate::repository::RepositoryType;
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;

pub struct Config {
    pub socket_addr: SocketAddr,
    pub log_level: String,
    pub cors_allowed_origin: String,
    pub repository_type: RepositoryType,
    pub sqlite_file_path: Option<String>,
}

impl Config {
    pub fn load() -> Config {
        let _ = dotenv();

        Config {
            socket_addr: env::var("SOCKET_ADDR")
                .expect("SOCKET_ADDR must be set")
                .parse()
                .expect("SOCKET_ADDR must be a valid SocketAddr"),
            log_level: env::var("LOG_LEVEL").unwrap_or("info".to_string()),
            cors_allowed_origin: env::var("CORS_ALLOWED_ORIGIN")
                .expect("CORS_ALLOWED_ORIGIN must be set"),
            repository_type: env::var("REPOSITORY_TYPE")
                .unwrap_or("hashmap".to_string())
                .parse()
                .expect("REPOSITORY_TYPE must be a valid RepositoryType"),
            sqlite_file_path: env::var("SQLITE_FILE_PATH")
                .into_iter()
                .filter(|s| !s.is_empty())
                .next(),
        }
    }
}
