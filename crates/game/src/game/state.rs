use hecs::World;

pub struct State {
    pub world: World,
    pub current_wave: u32,
    #[cfg(feature = "server")]
    pub serverstuff: u32,
}

impl State {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            current_wave: 0,
            #[cfg(feature = "server")]
            serverstuff: 0,
        }
    }
}
