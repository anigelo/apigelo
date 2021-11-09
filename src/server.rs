use std::fs;
use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;

pub fn media(media_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    fs::read_dir(media_path)?
        .map(|dir| {
            let dir = dir?;
            println!("{:?}", dir.file_name());
            Ok(dir.file_name().into_string().unwrap())
        })
        .collect()
}

pub fn episodes(media_path: &str, media_id: String) -> Result<Vec<String>, Box<dyn Error>> {
    let media_dir = fs::read_dir(media_path)?
        .find(|dir| dir.as_ref().unwrap().file_name() == OsString::from(&media_id));
    
    if let Some(media_dir) = media_dir {
        let allowed_extensions = vec!["mkv","mp4"];
        media_dir?.path().read_dir()?
            .map(|episode| Ok(episode?.path()))
            .filter(|episode: &Result<PathBuf, _>| episode.is_ok())
            .filter(|episode: &Result<PathBuf, _>| episode.as_ref().unwrap().extension().is_some())
            .filter(|episode: &Result<PathBuf, _>| {
                let extension = episode.as_ref().unwrap().extension().unwrap().to_str().unwrap();
                allowed_extensions.contains(&extension)
            })
            .map(|episode: Result<PathBuf, Box<dyn Error>>| {
                let episode = episode?.file_stem().unwrap().to_os_string();
                Ok(episode.into_string().unwrap())
            })
            .collect()
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound).into())
    }
}

pub fn playback(media_path: &str, media_id: String, episode: String) -> Option<PathBuf> {
    for dir in fs::read_dir(media_path).ok()? {
        let dir = dir.ok()?;
        let title = dir.file_name();
        println!("{:?}", title);
        if title == OsString::from(&media_id) {
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