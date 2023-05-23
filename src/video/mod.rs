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

fn sanitize_shell_characters(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('"', "\"\"")
        .replace('\'', "''")
        .replace('%', "\\%")
        .replace(':', "\\:")
        .replace('&', "\\&")
}

#[derive(Debug)]
pub struct DrawtextOptions {
    pub font: String,
    pub fontcolor: String,
    pub fontsize: u32,
    pub box_: bool,
    pub boxborderw: usize,
    pub boxcolor: String,
    pub x: String,
    pub y: String,
}

impl Default for DrawtextOptions {
    fn default() -> Self {
        Self {
            font: "Sans".to_string(),
            fontcolor: "white".to_string(),
            fontsize: 32,
            box_: false,
            boxborderw: 15,
            boxcolor: "black@0.8".to_string(),
            x: "(w-text_w)/2".to_string(),
            y: "(h-text_h)/2".to_string(),
        }
    }
}

impl DrawtextOptions {
    pub fn apply_to_text(&self, text: &str, start: f32, end: f32) -> String {
        // Replace special characters with escaped version
        let text = sanitize_shell_characters(text);
        // Wrap text to max width
        let text = wrap_text(&text, 60);

        let options = [
            // Font settings
            ("font", &format!("{}", self.font)),
            ("fontcolor", &self.fontcolor),
            ("fontsize", &self.fontsize.to_string()),
            // Text background
            ("box", &if self.box_ { "1" } else { "0" }.to_string()),
            ("boxborderw", &self.boxborderw.to_string()),
            ("boxcolor", &self.boxcolor),
            // Center text on canvas
            ("x", &self.x),
            ("y", &self.y),
            // Timing to display text
            ("enable", &format!("'between(t, {start}, {end})'")),
            // Prevent special characters in text from breaking command
            ("expansion", &"none".to_string()),
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
