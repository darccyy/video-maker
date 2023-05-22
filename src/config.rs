pub struct Config {
    pub voice: VoiceConfig,
    pub video: VideoConfig,
}

pub struct VoiceConfig {
    pub language: String,
    pub gender: String,
    pub pitch: f32,
    pub rate: f32,
}

pub struct VideoConfig {}
