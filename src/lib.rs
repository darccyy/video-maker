pub mod config;
pub mod paths;

mod reddit;
mod video;
mod voice;

use config::Config;
use paths::{check_assets_input, clean_assets_output};
use voice::{get_audio_duration, get_voice_bytes};

use std::{fs, process::Command, time::Duration};

pub fn create_video(config: Config) {
    // Clean and check assets directory
    clean_assets_output().expect("Failed to clean assets output");
    if let Some(missing_file) = check_assets_input().expect("Failed to check missing input files") {
        panic!("Input files/folders missing: {}", missing_file);
    };

    println!("\n======== CONTENT ========");

    let texts = reddit::get_posts(config.reddit).expect("Error fetching texts");

    println!("Successfully fetched content - {} lines", texts.len());

    println!("\n======== VOICE ==========");

    let mut voices = Vec::new();
    let mut duration_total = Duration::ZERO;
    for (i, text) in texts.iter().enumerate() {
        println!("Creating voice file for '{}'", text);

        let bytes = get_voice_bytes(&text).expect("Error fetching voice audio");

        let path = format!("{}/{}.mp3", paths::VOICES, i);
        fs::write(&path, &bytes).expect("Failed to write audio file of voice");

        let duration = get_audio_duration(&bytes).expect("Failed to parse audio duration");

        voices.push((path, duration, text));

        duration_total += duration;
    }

    println!("\n======== COMMAND ========");

    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-y", "-loglevel", "error", "-i", paths::BG]);

    for (path, ..) in &voices {
        cmd.args(["-i", &path]);
    }

    cmd.args(["-map", "0:v"]);
    for (i, _) in voices.iter().enumerate() {
        cmd.args(["-map", &format!("{}:a", i + 1)]);
    }

    let mut filter = String::new();
    for (i, _) in voices.iter().enumerate() {
        filter.push_str(&format!("[{}:a]", i + 1));
    }
    cmd.args([
        "-filter_complex",
        &format!("{}concat=n={}:v=0:a=1", filter, voices.len()),
    ]);

    let mut filters = Vec::new();
    let mut duration_total = 0.0;
    for (_, duration, text) in &voices {
        let start = duration_total;
        duration_total += duration.as_secs_f32();
        let end = duration_total;

        filters.push(video::text_filter(text, start, end));
    }
    cmd.args(["-vf", &filters.join(",")]);

    cmd.args([
        "-ss",
        "00:00:00",
        "-to",
        &video::timestamp_from_seconds(duration_total + 1.0),
    ]);

    cmd.args(["-q:v", "0"]);
    // cmd.args(["-c:v", "copy"]);
    cmd.arg(paths::FINAL);

    println!("{:#?}", cmd);
    println!(
        "ffmpeg {}",
        cmd.get_args()
            .map(|x| x.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ")
    );

    println!("\n======== RESULT ==========");
    println!("Rendering with ffmpeg...");

    let result = cmd.output().expect("Run command");

    if !result.stderr.is_empty() {
        eprintln!("FFMPEG Error");
        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
        std::process::exit(1);
    } else {
        println!("\x1b[1mSuccess!\x1b[0m\n");
    }
}
