mod media;
mod server;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder, Result, error};
use crate::media::KitsuMediaCollection;
use std::io::ErrorKind;
use std::env;

const DEFAULT_MEDIA_PATH: &str = "/media";

#[get("/media")]
async fn get_media() -> impl Responder {
    match server::media(&get_media_path()) {
        Ok(media) => KitsuMediaCollection { data: Some(media) },
        _ => KitsuMediaCollection { data: None }
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
    HttpServer::new(|| App::new().service(get_media).service(playback))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

fn get_media_path() -> String {
    env::var("MEDIA_PATH").unwrap_or(String::from(DEFAULT_MEDIA_PATH))
}
