use std::collections::HashMap;

use macroquad::{
    audio::{Sound, load_sound},
    prelude::*,
};

pub struct AudioHandler {
    pub sounds: HashMap<String, Sound>,
}

impl AudioHandler {
    pub async fn load_sounds() -> anyhow::Result<Self> {
        Ok(Self {
            sounds: HashMap::from([(
                "explosion".to_owned(),
                load_sound("assets/sfx/explosion.wav").await?,
            )]),
        })
    }
}
