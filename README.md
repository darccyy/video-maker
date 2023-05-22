# Video Maker

A Rust CLI program to automatically create a bad Youtube video, using the Reddit API, a free TTS API, and FFMpeg.

# Requirements

- `ffmpeg` must be installed, with `--enable-libfreetype` and `--enable-libfontconfig` flags configured.
- `./assets/in/bg.mp4` must exist in the working directory

Note: Both the Reddit and TTS API's do *not* need a key, as they are public.

