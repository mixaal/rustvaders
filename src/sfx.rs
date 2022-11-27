use std::thread;

use rusty_audio::Audio;

const LASER: &str = "LASER";
const ALIEN_MISSILE: &str = "ALIEN_MISSILE";
const BGM: &str = "BGM";

pub struct Sfx {
    _audio: Audio,
}

impl Sfx {
    pub fn new() -> Self {
        let mut _audio = Audio::new();
       
        _audio.add(LASER, "sfx/laser.mp3");
        _audio.add(ALIEN_MISSILE, "sfx/alien_missile.mp3");
       
        Self { _audio, }
    }

    pub fn laser(&mut self) {
        self._audio.play(LASER);
    }

    pub fn alien_missile(&mut self) {
        self._audio.play(ALIEN_MISSILE);
    }

    pub fn bgm(&mut self) {
        thread::spawn(move || {
            let mut audio = Audio::new();
            let have_bgm = std::path::Path::new("sfx/running-against-clock.mp3").exists();
            // I don't have rights to distribute this sound
            // Elliot Holmes - Running Against the Clock - https://www.epidemicsound.com/track/8bc4l5LWYf/
            if !have_bgm {
                return ;
            }
            audio.add(BGM, "sfx/running-against-clock.mp3");
            loop {
                audio.play(BGM);
                audio.wait();
            }
        });
    }
}
