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

pub fn text_filter(text: &str, start: f32, end: f32) -> String {
    // Wrap text to max width
    let text = wrap_text(text, 60);

    let options = [
        // Font settings
        ("font", "'Serif'"),
        ("fontcolor", "white"),
        ("fontsize", "28"),
        // Text background
        ("box", "1"),
        ("boxborderw", "5"),
        ("boxcolor", "black@0.6"),
        // Center text on canvas
        ("x", "(w-text_w)/2"),
        ("y", "(h-text_h)/2"),
        // Timing to display text
        ("enable", &format!("'between(t, {start}, {end})'")),
        // Text to render
        ("text", &format!("'{}'", text)),
    ];

    // Convert to `key=value` syntax
    let options: Vec<_> = options
        .into_iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect();
    // Create filter
    format!("drawtext={}", options.join(":"))
}

/// Wrap text to fit within a maximum width (number of characters)
fn wrap_text(text: &str, max_width: usize) -> String {
    // Split into 'words'
    // Words longer than `max_width` will be split into two 'words', with a dash appended to all
    // non-final words
    let mut words = Vec::new();
    for word in text.split(' ') {
        // Divide word into chunks, with a max width
        let chars: Vec<char> = word.chars().collect();
        let chunks = chars.chunks(max_width - 1);
        // Calculate amount of chunks, without consuming iterator
        let chunk_count = chars.len() / (max_width - 1);

        for (i, chunk) in chunks.enumerate() {
            // Convert to string
            let mut chunk = chunk.iter().collect::<String>();
            // Add dash, if not final chunk
            if i + 1 <= chunk_count {
                chunk.push('-');
            }
            // Add to words
            words.push(chunk);
        }
    }

    // Create lines of text
    let mut lines = Vec::new();
    let mut line = String::new();

    for word in words {
        // Length of line, if word is added
        let future_len = if line.is_empty() {
            // Only word length
            // This should never be longer than `max_width`, due to chunking earlier
            word.len()
        } else {
            // Line length, with new word, and another space character
            line.len() + 1 + word.len()
        };

        // Create new line, if adding word to line would otherwise make line longer than `max_width`
        if future_len >= max_width {
            lines.push(line);
            line = String::new();
        }

        // Add space, if not first word in line
        if !line.is_empty() {
            line.push(' ');
        }
        // Add word to line
        line.push_str(&word);
    }

    // Join lines
    lines.push(line);
    lines.join("\n")
}
