use hecs::World;

pub struct State {
    pub world: World,
    pub current_wave: u32,
}

impl State {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            current_wave: 0,
        }
    }
}
