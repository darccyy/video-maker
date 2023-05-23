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
    TEMP   = "/tmp/video-maker"
    VOICES = TEMP/"voices"
    FILTER = TEMP/"filter.txt"
);

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
