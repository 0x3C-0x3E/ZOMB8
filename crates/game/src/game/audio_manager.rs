use std::collections::HashMap;

use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, Semitones,
    sound::static_sound::StaticSoundData,
};
use macroquad::rand;

#[derive(Hash, PartialEq, Eq)]
pub enum SoundKind {
    Explosion,
    Shoot,
    HealthPack,
    Hit,
}

pub struct AudioHandler {
    pub manager: AudioManager,
    pub sounds: HashMap<SoundKind, StaticSoundData>,
}

impl AudioHandler {
    pub async fn load_sounds() -> Self {
        let explosion = (
            SoundKind::Explosion,
            StaticSoundData::from_file("assets/sfx/explosion.wav").unwrap(),
        );

        let shoot = (
            SoundKind::Shoot,
            StaticSoundData::from_file("assets/sfx/shoot.wav").unwrap(),
        );

        let health_box = (
            SoundKind::HealthPack,
            StaticSoundData::from_file("assets/sfx/health_box.wav").unwrap(),
        );

        let hit = (
            SoundKind::Hit,
            StaticSoundData::from_file("assets/sfx/hit.wav").unwrap(),
        );

        Self {
            manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap(),
            sounds: HashMap::from([explosion, shoot, health_box, hit]),
        }
    }

    pub fn play_sound_once(&mut self, id: SoundKind) {
        let sound = self
            .sounds
            .get(&id)
            .unwrap()
            .clone()
            .playback_rate(Semitones(rand::gen_range(-2.0, 2.0)));

        let _ = self.manager.play(sound);
    }
}
