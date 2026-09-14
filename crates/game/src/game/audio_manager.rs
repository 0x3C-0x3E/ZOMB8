use std::collections::HashMap;

use macroquad::audio::{Sound, load_sound, play_sound_once};

pub struct AudioManager {
    pub sounds: HashMap<String, Sound>,
}

impl AudioManager {
    pub async fn load_sounds() -> Self {
        let explosion = (
            "explosion".to_owned(),
            load_sound("assets/sfx/explosion.wav").await.unwrap(),
        );

        let shoot = (
            "shoot".to_owned(),
            load_sound("assets/sfx/shoot.wav").await.unwrap(),
        );

        let health_box = (
            "health_box".to_owned(),
            load_sound("assets/sfx/health_box.wav").await.unwrap(),
        );

        Self {
            sounds: HashMap::from([explosion, shoot, health_box]),
        }
    }

    pub fn play_sound_once(&self, id: &str) {
        play_sound_once(self.sounds.get(id).unwrap_or_else(|| {
            panic!("invalid sound id {id}");
        }));
    }
}
