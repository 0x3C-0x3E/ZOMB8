use hecs::World;

pub struct State {
    pub world: World,
}

impl State {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
}
