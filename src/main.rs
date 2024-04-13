use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use log;
use r2d2_sqlite::SqliteConnectionManager;
use std::io;
use tiny_url::config::Config;
use tiny_url::core::OwnedRepository;
use tiny_url::hash_map_repository::HashMapRepository;
use tiny_url::link::Link;
use tiny_url::link_sqlite_repository::LinkSqliteRepository;
use tiny_url::repository::RepositoryType;
use tiny_url::service;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = Config::load();

    let _ = pretty_env_logger::formatted_builder()
        .default_format()
        .parse_filters(config.log_level.as_str())
        .init();

    let repo: OwnedRepository<Link> = match config.repository_type {
        RepositoryType::HashMap => {
            log::info!("using HashMap as repository");
            Box::new(HashMapRepository::new())
        }
        RepositoryType::Sqlite => {
            let manager = match config.sqlite_file_path {
                Some(ref path) => {
                    log::info!("using SQLite file({}) as repository", path);
                    SqliteConnectionManager::file(path)
                }
                None => {
                    log::info!("using SQLite memory as repository");
                    SqliteConnectionManager::memory()
                }
            };
            let pool = r2d2::Pool::new(manager).unwrap();
            Box::new(LinkSqliteRepository::new(pool))
        }
    };
    let link_service = service::ShortLinkService::new(repo);

    log::info!("starting HTTP server at http://{}", config.socket_addr);
    log::info!(
        "please visit http://{}/static/index.html",
        config.socket_addr
    );

    let link_service = web::Data::new(link_service);
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&config.cors_allowed_origin)
                    .allowed_methods(vec!["GET", "POST"])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .app_data(link_service.clone())
            .service(tiny_url::route::get_short_link)
            .service(tiny_url::route::create_short_link)
            .service(actix_files::Files::new("/static", "./static").index_file("index.html"))
    })
    .bind(config.socket_addr)?
    .workers(1)
    .run()
    .await
}
