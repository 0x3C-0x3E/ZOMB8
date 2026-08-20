use crate::ecs::systems::rendering::RenderingState;
use hecs::World;

pub struct State {
    pub world: World,
    pub rendering_state: RenderingState,
}

impl State {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            rendering_state: RenderingState::new(),
        }
    }
}
