mod media;
mod server;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder, Result, error};
use crate::media::{MediaCollection};
use std::io::ErrorKind;
use std::env;

const DEFAULT_MEDIA_PATH: &str = "/media";

#[get("/media")]
async fn get_media() -> impl Responder {
    println!("{}", &get_media_path());
    match server::media(&get_media_path()) {
        Ok(media) => MediaCollection { data: Some(media) },
        _ => MediaCollection { data: None }
    }
}

#[get("/media/{media_id}/episodes")]
async fn get_episodes(web::Path(media_id): web::Path<String>) -> impl Responder {
    match server::episodes(&get_media_path(), media_id) {
        Ok(media) => MediaCollection { data: Some(media) },
        _ => MediaCollection { data: None }
    }
}

#[get("/media/{media_id}/episodes/{episode}/playback")]
async fn playback(web::Path((media_id, episode)): web::Path<(String, String)>) -> Result<NamedFile> {
    let episode_path = server::playback(&get_media_path(), media_id, episode)
        .ok_or(std::io::Error::from(ErrorKind::NotFound))
        .map_err(|e| error::ErrorNotFound(e))?;

    Ok(NamedFile::open(episode_path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_media).service(get_episodes).service(playback))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

fn get_media_path() -> String {
    env::var("MEDIA_PATH").unwrap_or(String::from(DEFAULT_MEDIA_PATH))
}
