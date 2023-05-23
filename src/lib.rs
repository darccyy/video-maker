pub mod config;
pub mod paths;

pub mod reddit;
mod video;
mod voice;

pub use paths::clean_assets_output;

use config::Config;
use voice::get_voice_bytes;

use std::{fs, process::Command, time::Duration};

use crate::video::DrawtextOptions;

pub trait ToTextFrames {
    fn to_text_frames(self) -> Vec<String>;
}

impl<T: ToTextFrames> ToTextFrames for Vec<T> {
    fn to_text_frames(self) -> Vec<String> {
        self.into_iter()
            .map(ToTextFrames::to_text_frames)
            .flatten()
            .collect()
    }
}

pub fn create_video(texts: Vec<String>, config: &Config) {
    println!("\n======== VOICE ==========");

    let mut voices = Vec::new();
    let mut duration_total = Duration::ZERO;
    for (i, text) in texts.iter().enumerate() {
        println!("Creating voice line: {}/{}", i + 1, texts.len());

        let (bytes, duration) =
            get_voice_bytes(&text, &config.voice).expect("Error fetching voice audio");

        let path = format!("{}/{}.mp3", paths::VOICES, i);
        fs::write(&path, &bytes).expect("Failed to write audio file of voice");

        voices.push((path, duration, text));

        duration_total += duration;
    }

    println!("\n======== COMMAND ========");

    // Create ffmpeg command, with some basic params, and input video
    let mut cmd = Command::new("ffmpeg");
    if config.video.overwrite {
        cmd.arg("-y");
    }
    cmd.args(["-loglevel", "error", "-i", &config.source.background]);

    // Add audio inputs
    for (path, ..) in &voices {
        cmd.args(["-i", &path]);
    }

    // Create filters
    // Audio concatenation
    let mut filter = String::new();
    let mut duration_total = 0.0;
    for (i, _) in voices.iter().enumerate() {
        filter.push_str(&format!("[{}:a]", i + 1));
    }
    filter.push_str(&format!("concat=n={}:v=0:a=1;", voices.len()));
    // Drawtext video filters
    let drawtext_options = DrawtextOptions {
        font: "Serif".to_string(),
        box_: true,
        ..Default::default()
    };
    for (i, (_, duration, text)) in voices.iter().enumerate() {
        let start = duration_total;
        duration_total += duration.as_secs_f32();
        let end = duration_total;

        if i > 0 {
            filter.push(',');
        }
        filter.push_str(&drawtext_options.apply_to_text(text, start, end));
    }
    // Apply watermark
    if let Some(watermark) = &config.source.watermark {
        let drawtext_options = DrawtextOptions {
            x: "w*0.8-text_w/2".to_string(),
            y: "h*0.3-text_h/2".to_string(),
            ..Default::default()
        };
        filter.push_str(&drawtext_options.apply_to_text(watermark, 0.0, duration_total));
    }
    // Save to file and pass filepath as argument
    fs::write(paths::FILTER, filter).expect("Failed to write temporary filter script");
    cmd.args(["-filter_complex_script", paths::FILTER]);

    // Trim video to duration of all audio
    cmd.args([
        "-ss",
        "00:00:00",
        "-to",
        &video::timestamp_from_seconds(duration_total + 2.0),
    ]);

    // Lossless video, without copy
    cmd.args(["-q:v", "0"]);
    // Output video
    cmd.arg(&config.video.out);

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
