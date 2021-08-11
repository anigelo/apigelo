use std::fs;
use crate::media::KitsuMedia;
use std::error::Error;
use std::path::PathBuf;

const METADATA_FILE: &str = "metadata.anistk";

pub fn media(media_path: &str) -> Result<Vec<KitsuMedia>, Box<dyn Error>> {
    fs::read_dir(media_path)?
        .map(|dir| fs::read_to_string(dir?.path().join(METADATA_FILE)))
        .map(|text_file| Ok(serde_json::from_str::<KitsuMedia>(text_file?.as_str())?))
        .collect()
}

pub fn playback(media_path: &str, media_id: String, episode: String) -> Option<PathBuf> {
    for dir in fs::read_dir(media_path).ok()? {
        let dir = dir.ok()?;
        let text_file = fs::read_to_string(dir.path().join(METADATA_FILE)).ok()?;
        let title = serde_json::from_str::<KitsuMedia>(text_file.as_str()).ok()?;
        println!("{:?}", title);
        if title.id? == media_id {
            println!("found!");
            return dir.path().read_dir().ok()?
                .map(|ep| ep.unwrap().path())
                .find(|ep| {
                    println!("{:?} - {}", ep.file_stem(), episode);
                    ep.file_stem().unwrap().to_str().unwrap() == episode
                });
        }
    }

    None
}