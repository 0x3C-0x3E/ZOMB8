use std::collections::HashMap;

use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, Semitones,
    sound::static_sound::StaticSoundData,
};
use macroquad::rand;

pub struct AudioHandler {
    pub manager: AudioManager,
    pub sounds: HashMap<String, StaticSoundData>,
}

impl AudioHandler {
    pub async fn load_sounds() -> Self {
        let explosion = (
            "explosion".to_owned(),
            StaticSoundData::from_file("assets/sfx/explosion.wav").unwrap(),
        );

        let shoot = (
            "shoot".to_owned(),
            StaticSoundData::from_file("assets/sfx/shoot.wav").unwrap(),
        );

        let health_box = (
            "health_box".to_owned(),
            StaticSoundData::from_file("assets/sfx/health_box.wav").unwrap(),
        );

        Self {
            manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap(),
            sounds: HashMap::from([explosion, shoot, health_box]),
        }
    }

    pub fn play_sound_once(&mut self, id: &str) {
        let sound = self
            .sounds
            .get(id)
            .unwrap()
            .clone()
            .playback_rate(Semitones(rand::gen_range(-2.0, 2.0)));

        let _ = self.manager.play(sound);
    }
}
