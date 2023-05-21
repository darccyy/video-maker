use std::fs;

use video_maker::{
    check_assets_input, clean_assets_output, fetch_voice_bytes, get_audio_duration, get_texts,
    paths,
};

fn main() {
    // Clean and check assets directory
    clean_assets_output().expect("Failed to clean assets output");
    if let Some(missing_file) = check_assets_input().expect("Failed to check missing input files") {
        panic!("Input files/folders missing: {}", missing_file);
    };

    let texts = get_texts().expect("Failed to read texts file");
    let mut voice_durations = Vec::new();

    for (i, text) in texts.iter().enumerate() {
        println!("Creating voice file for '{}'", text);

        let bytes = fetch_voice_bytes(&text).expect("Error fetching voice audio");

        let duration = get_audio_duration(&bytes).expect("Failed to parse audio duration");
        println!("Duration: {}ms", duration.as_millis());
        voice_durations.push(duration);

        let path = format!("{}/{}.mp4", paths::VOICES, i);
        fs::write(&path, &bytes).expect("Failed to write audio file of voice");
    }
}
