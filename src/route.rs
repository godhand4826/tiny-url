use crate::service::{CreateLinkError, GetLinkError, ShortLinkService};
use actix_web::{get, http, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateLinkRequest {
    url: String,
}

#[post("/")]
async fn create_short_link(
    data: web::Data<ShortLinkService>,
    req: web::Json<CreateLinkRequest>,
) -> impl Responder {
    let result = data.create_short_link(req.url.clone());
    match result {
        Ok(link) => HttpResponse::Ok().body(link.id),
        Err(CreateLinkError::InvalidUrl(err)) => HttpResponse::BadRequest().body(err.to_string()),
        Err(CreateLinkError::MaxAttemptExceeded(_)) => HttpResponse::ServiceUnavailable().finish(),
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{link_id}")]
async fn get_short_link(
    path: web::Path<String>,
    data: web::Data<ShortLinkService>,
) -> impl Responder {
    let result = data.get_link_by_id(&path.into_inner());

    match result {
        Ok(link) => HttpResponse::Found()
            .insert_header((http::header::LOCATION, link.url))
            .finish(),
        Err(GetLinkError::NotFound(id)) => {
            HttpResponse::NotFound().body(format!("short link {} not found", id))
        }
        Err(GetLinkError::Internal(err)) => {
            println!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
