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
    TEXTS  = IN/"texts.txt"
    VOICES = TEMP/"voices"
);
