use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use log;
use std::io;
use std::sync::Mutex;
use tiny_url::config::Config;
use tiny_url::core::OwnedRepository;
use tiny_url::link::Link;
use tiny_url::repository;
use tiny_url::service;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = Config::load();

    let _ = pretty_env_logger::formatted_builder()
        .default_format()
        .parse_filters(config.log_level.as_str())
        .init();

    let repo: OwnedRepository<Link> = Box::new(repository::HashMapRepository::new());
    let link_service = Mutex::new(service::ShortLinkService::new(repo));
    let link_service = web::Data::new(link_service);

    log::info!("starting HTTP server at {}", config.socket_addr);

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
    })
    .bind(config.socket_addr)?
    .workers(1)
    .run()
    .await
}
