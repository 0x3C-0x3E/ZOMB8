use glam::Vec2;
use hecs::World;

use game::ecs::{
    entities::{tile::Tile, zombie::Zombie},
    transform::Position,
};

use crate::server::Server;

fn _get_level_constraints(world: &World) -> (Vec2, Vec2) {
    let mut max = Vec2::ZERO;
    let mut min = Vec2::ZERO;
    for (_t, pos) in world.query::<(&Tile, &Position)>().iter() {
        if pos.x > max.x {
            max.x = pos.x;
        } else if pos.x < min.x {
            min.x = pos.x;
        }

        if pos.y > max.y {
            max.y = pos.y;
        } else if pos.y < min.y {
            min.y = pos.y;
        }
    }

    let max = Vec2::new(max.x + 16.0, max.y + 16.0);
    (min, max)
}
impl Server {
    pub async fn spawn_wave_system(&mut self) {
        let zombies_count = self.state.world.query::<&Zombie>().into_iter().len();
        if zombies_count == 0 {
            self.spawn_wave().await;
        }
    }

    async fn spawn_wave(&mut self) {
        let _ = self.spawn_zombie(Position { x: 40.0, y: 0.0 }).await;
    }
}
