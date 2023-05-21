use std::{io::Cursor, time::Duration};

pub fn fetch_voice_bytes(text: &str) -> Result<Vec<u8>, reqwest::Error> {
    let language = "en-GB";
    let gender = "male";

    let url = format!("https://texttospeech.responsivevoice.org/v1/text:synthesize?text={text}&lang={language}&engine=g1&name=&pitch=0.5&rate=0.5&volume=1&key=kvfbSITh&gender={gender}");

    let response = reqwest::blocking::get(&url)?;

    let bytes = response.bytes()?;

    Ok(bytes.to_vec())
}

pub fn get_audio_duration(bytes: &[u8]) -> Result<Duration, mp3_duration::MP3DurationError> {
    let mut cursor = Cursor::new(bytes);
    let duration = mp3_duration::from_read(&mut cursor)?;
    Ok(duration)
}
