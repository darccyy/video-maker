use regex::Regex;
use std::{io::Cursor, time::Duration};

use crate::config;

type BytesAndDuration = Result<(Vec<u8>, Duration), Box<dyn std::error::Error>>;

pub fn get_voice_bytes(text: &str, config: &config::Voice) -> BytesAndDuration {
    let config::Voice {
        language,
        gender,
        pitch,
        rate,
    } = config;

    let text = remove_emojis(text);

    let url = format!("https://texttospeech.responsivevoice.org/v1/text:synthesize?text={text}&lang={language}&engine=g1&name=&pitch={pitch}&rate={rate}&volume=1&key=kvfbSITh&gender={gender}");

    let attempt = || -> BytesAndDuration {
        let response = reqwest::blocking::get(&url)?;

        let bytes = response.bytes()?;

        let duration = get_audio_duration(&bytes)?;

        Ok((bytes.to_vec(), duration))
    };

    const MAX_ATTEMPTS: usize = 10;

    let mut i = 0;
    loop {
        i += 1;

        match attempt() {
            Ok(value) => return Ok(value),

            Err(err) => {
                eprintln!(
                    "[warning] (Attempt {i}/{MAX_ATTEMPTS}): Failed to create voice line - {err:?}"
                );

                if i >= MAX_ATTEMPTS {
                    return Err(err);
                }
            }
        };
    }
}

fn remove_emojis(text: &str) -> String {
    let emoji_regex = Regex::new(r#"\p{Emoji}"#).unwrap();
    emoji_regex.replace_all(text, "").to_string()
}

pub fn get_audio_duration(bytes: &[u8]) -> Result<Duration, mp3_duration::MP3DurationError> {
    let mut cursor = Cursor::new(bytes);
    let duration = mp3_duration::from_read(&mut cursor)?;
    Ok(duration)
}
