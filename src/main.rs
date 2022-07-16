mod database;
mod config;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header;
use dotenv::dotenv;

use crate::database::*;

#[get("/media")]
async fn all_media() -> impl Responder {
    let media = get_all_media().await;

    if media.is_empty() {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::Ok().json(media)
    }
}

#[get("/media/{media_id}")]
async fn media_details(path: web::Path<String>) -> impl Responder {
    let media = get_media(path.into_inner()).await;

    match media {
        Some(anime) => HttpResponse::Ok().json(anime),
        None => HttpResponse::NotFound().finish()
    }
}

#[get("/media/{media_id}/seasons")]
async fn media_seasons(path: web::Path<String>) -> impl Responder {
    let media = get_media_seasons(path.into_inner()).await;

    if media.is_empty() {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::Ok().json(media)
    }
}

#[get("/media/{media_id}/seasons/{season_id}")]
async fn media_season_details(path: web::Path<(String, u32)>) -> impl Responder {
    let (media_id, season_id) = path.into_inner();

    let season = get_season(media_id, season_id).await;
    match season {
        Some(season) => HttpResponse::Ok().json(season),
        None => HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin(&config::cors_origin())
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(all_media)
            .service(media_details)
            .service(media_seasons)
            .service(media_season_details)
            .service(actix_files::Files::new("/static", config::media_path())
                .show_files_listing())
    })
        .bind(("0.0.0.0", config::http_port()))?
        .run()
        .await
}
