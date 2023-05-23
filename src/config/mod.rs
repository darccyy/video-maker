use serde::Deserialize;

pub fn parse(file: &str) -> Config {
    let builder: ConfigBuilder = toml::from_str(&file).expect("Failed to parse config toml file");
    builder.build()
}

trait Builder {
    type Output;
    fn build(self) -> Self::Output;
}

#[derive(Debug)]
pub struct Config {
    pub video: Video,
    pub source: Source,
    pub reddit: Reddit,
    pub voice: Voice,
}

#[derive(Debug, Deserialize)]
struct ConfigBuilder {
    video: VideoBuilder,
    source: SourceBuilder,
    reddit: Option<RedditBuilder>,
    voice: Option<VoiceBuilder>,
}

impl Builder for ConfigBuilder {
    type Output = Config;
    fn build(self) -> Self::Output {
        Self::Output {
            video: self.video.build(),
            source: self.source.build(),
            reddit: self.reddit.unwrap_or_default().build(),
            voice: self.voice.unwrap_or_default().build(),
        }
    }
}

#[derive(Debug)]
pub struct Video {
    pub out: String,
}

#[derive(Debug, Deserialize)]
struct VideoBuilder {
    out: String,
}

impl Builder for VideoBuilder {
    type Output = Video;
    fn build(self) -> Self::Output {
        Self::Output { out: self.out }
    }
}

#[derive(Debug)]
pub struct Source {
    pub background: String,
    pub watermark: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SourceBuilder {
    background: String,
    watermark: Option<String>,
}

impl Builder for SourceBuilder {
    type Output = Source;
    fn build(self) -> Self::Output {
        Self::Output {
            background: self.background,
            watermark: self.watermark,
        }
    }
}

#[derive(Debug)]
pub struct Reddit {
    pub subreddit: String,
    pub sort: String,
    pub time: String,
    pub comments: bool,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Default)]
struct RedditBuilder {
    subreddit: Option<String>,
    sort: Option<String>,
    time: Option<String>,
    comments: Option<bool>,
    limit: Option<usize>,
}

impl Builder for RedditBuilder {
    type Output = Reddit;
    fn build(self) -> Self::Output {
        Self::Output {
            subreddit: self.subreddit.unwrap_or(String::from("askreddit")),
            sort: self.sort.unwrap_or(String::from("top")),
            time: self.time.unwrap_or(String::from("month")),
            comments: self.comments.unwrap_or(true),
            limit: self.limit,
        }
    }
}

#[derive(Debug)]
pub struct Voice {
    pub language: String,
    pub gender: String,
    pub pitch: f32,
    pub rate: f32,
}

#[derive(Debug, Deserialize, Default)]
struct VoiceBuilder {
    language: Option<String>,
    gender: Option<String>,
    pitch: Option<f32>,
    rate: Option<f32>,
}

impl Builder for VoiceBuilder {
    type Output = Voice;
    fn build(self) -> Self::Output {
        Self::Output {
            language: self.language.unwrap_or(String::from("en-GB")),
            gender: self.gender.unwrap_or(String::from("male")),
            pitch: self.pitch.unwrap_or(0.5),
            rate: self.rate.unwrap_or(0.5),
        }
    }
}
