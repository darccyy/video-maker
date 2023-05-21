use std::{fs, process::Command};

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

    println!("\n======== VOICE ==========");

    let texts = get_texts().expect("Failed to read texts file");

    let mut voices = Vec::new();

    for (i, text) in texts.iter().enumerate() {
        println!("Creating voice file for '{}'", text);

        let bytes = fetch_voice_bytes(&text).expect("Error fetching voice audio");

        let path = format!("{}/{}.mp4", paths::VOICES, i);
        fs::write(&path, &bytes).expect("Failed to write audio file of voice");

        let duration = get_audio_duration(&bytes).expect("Failed to parse audio duration");
        println!("Duration: {}ms", duration.as_millis());

        voices.push((path, duration));
    }

    println!("\n======== COMMAND ========");

    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-y", "-loglevel", "error", "-i", paths::BG]);

    for (path, _) in &voices {
        cmd.args(["-i", &path]);
    }

    cmd.args(["-map", "0:v"]);

    for (i, _) in voices.iter().enumerate() {
        cmd.args(["-map", &format!("{}:a", i + 1)]);
    }

    cmd.arg(paths::FINAL);

    println!("{:#?}", cmd);
    println!("\n======== RESULT ==========");

    let result = cmd.output().expect("Run command");

    if !result.stderr.is_empty() {
        eprintln!("FFMPEG Error");
        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
        std::process::exit(1);
    } else {
        println!("\x1b[1mSuccess!\x1b[0m\n");
    }
}
