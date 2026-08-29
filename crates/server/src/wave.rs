use glam::Vec2;
use hecs::World;
use rand::random_range;

use game::ecs::{
    entities::{tile::Tile, zombie::Zombie},
    transform::Position,
};

use crate::server::Server;

fn get_level_constraints(world: &World) -> (Vec2, Vec2) {
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

fn is_occupied(world: &World, pos: &Position) -> bool {
    world
        .query::<&Position>()
        .with::<&Tile>()
        .into_iter()
        .find(|t_pos| *t_pos == pos)
        .is_some()
}

impl Server {
    pub async fn spawn_wave_system(&mut self) {
        let zombies_count = self.state.world.query::<&Zombie>().into_iter().len();
        if zombies_count == 0 {
            self.spawn_wave().await;
        }
    }

    async fn spawn_wave(&mut self) {
        let level_constrains = get_level_constraints(&self.state.world);

        let _ = self
            .spawn_zombie(self.choose_position(&level_constrains))
            .await;
    }

    fn choose_position(&self, level_constrains: &(Vec2, Vec2)) -> Position {
        loop {
            let pos = Position {
                x: random_range(level_constrains.0.x..level_constrains.1.x),
                y: random_range(level_constrains.0.y..level_constrains.1.y),
            };

            if !is_occupied(&self.state.world, &pos) {
                return pos;
            }
        }
    }
}
