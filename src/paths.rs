use std::{fs, io, path::Path};

use const_format::concatcp;

const TEMP: &str = concatcp!("/tmp/", env!("CARGO_PKG_NAME"));

pub const VOICES: &str = concatcp!(TEMP, "/voices");
pub const FILTER: &str = concatcp!(TEMP, "/filter.txt");

/// Folders that may already exist, they should be recreated empty
pub fn clean_assets_output() -> Result<(), io::Error> {
    let folders_to_recreate = [TEMP, VOICES];
    for path in folders_to_recreate {
        if Path::new(path).exists() {
            fs::remove_dir_all(path)?;
        }
        fs::create_dir(path)?;
    }
    Ok(())
}
