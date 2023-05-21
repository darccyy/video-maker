use std::{fs, io, path::Path};

use const_format::concatcp;

/// Create constants for paths
macro_rules! define_paths {
    ( $(
        $name: ident =
            $($parent: ident /)? $file: literal
    )* ) => { $(
        /// Constant path for assets file or folder
        pub const $name: &str =
            concatcp!( $($parent, "/", )? $file );
    )* };
}

define_paths!(
    ASSETS = "assets"
    IN     = ASSETS/"in"
    OUT    = ASSETS/"out"
    TEMP   = ASSETS/"temp"
    BG     = IN/"bg.mp4"
    FINAL  = OUT/"final.mp4"
    VOICES = TEMP/"voices"
    FILTER = TEMP/"filter.txt"
);

/// Folders that may already exist, they should be recreated empty
pub fn clean_assets_output() -> Result<(), io::Error> {
    let folders_to_recreate = [OUT, TEMP, VOICES];
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
    let files_must_exist = [ASSETS, IN, BG];
    for path in files_must_exist {
        if !Path::new(path).exists() {
            return Ok(Some(path));
        }
    }
    Ok(None)
}
