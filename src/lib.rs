pub mod paths;

mod fetch;
mod voice;

pub use voice::{fetch_voice_bytes, get_audio_duration};

use std::{fs, io, path::Path};

/// Folders that may already exist, they should be recreated empty
pub fn clean_assets_output() -> Result<(), io::Error> {
    let folders_to_recreate = [paths::OUT, paths::TEMP, paths::VOICES];
    for path in folders_to_recreate {
        if Path::new(path).exists() {
            fs::remove_dir_all(path)?;
        }
        fs::create_dir(path)?;
    }
    Ok(())
}

/// Folders and files that must already exist
pub fn check_assets_input() -> Result<Option<&'static str>, io::Error> {
    let files_must_exist = [paths::ASSETS, paths::IN, paths::TEXTS, paths::BG];
    for path in files_must_exist {
        if !Path::new(path).exists() {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

pub fn get_texts() -> Result<Vec<String>, io::Error> {
    Ok(fs::read_to_string(paths::TEXTS)?
        .lines()
        .filter(|string| !string.is_empty())
        .map(String::from)
        .collect())
}

/// Format timestamp (hh:mm:ss) from time in seconds
///
/// TODO Add milliseconds
pub fn timestamp_from_seconds(seconds: f32) -> String {
    let mut seconds = seconds as u32;

    let mut minutes = seconds / 60;
    seconds = seconds % 60;

    let hours = minutes / 60;
    minutes = minutes % 60;

    format!(
        "{hh}:{mm}:{ss}",
        hh = hours,
        mm = leading_zeros(minutes, 2),
        ss = leading_zeros(seconds, 2)
    )
}

/// Add leading zero to number, if less than desired digit length
fn leading_zeros(number: u32, length: usize) -> String {
    let number = number.to_string();
    if number.len() < length {
        "0".repeat(length - number.len()) + &number
    } else {
        number
    }
}
