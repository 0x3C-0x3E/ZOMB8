use std::collections::HashMap;

use kira::{
    AudioManager, AudioManagerSettings, Decibels, DefaultBackend, Easing, Mapping, Semitones,
    Value,
    effect::reverb::ReverbBuilder,
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
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
    pub tweener: TweenerHandle,
    pub sounds: HashMap<SoundKind, StaticSoundData>,
    pub playing: HashMap<SoundKind, StaticSoundHandle>,
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

        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        let tweener = manager
            .add_modulator(TweenerBuilder { initial_value: 0.7 })
            .unwrap();

        Self {
            manager,
            tweener,
            sounds: HashMap::from([explosion, shoot, health_box, hit]),
            playing: HashMap::new(),
        }
    }

    pub fn get_processed_sound(&self, sound: StaticSoundData) -> StaticSoundData {
        sound
            .playback_rate(Semitones(rand::gen_range(-2.0, 2.0)))
            .volume(Value::from_modulator(
                self.tweener.id(),
                Mapping {
                    input_range: (0.0, 1.0),
                    output_range: (Decibels::SILENCE, Decibels::IDENTITY),
                    easing: Easing::Linear,
                },
            ))
    }

    pub fn play_sound_once(&mut self, id: SoundKind) {
        let sound = self.sounds.get(&id).unwrap().clone();

        let sound = self.get_processed_sound(sound);

        let _ = self.manager.play(sound);
    }

    pub fn try_play_sound(&mut self, id: SoundKind) {
        if let Some(handle) = self.playing.get(&id) {
            if matches!(
                handle.state(),
                kira::sound::PlaybackState::Playing
                    | kira::sound::PlaybackState::Resuming
                    | kira::sound::PlaybackState::WaitingToResume
            ) {
                return;
            }
        }

        let sound = self.sounds.get(&id).unwrap().clone();
        let sound = self.get_processed_sound(sound);
        let handle = self.manager.play(sound).unwrap();

        self.playing.insert(id, handle);
    }
}
