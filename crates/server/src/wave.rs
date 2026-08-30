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

#[derive(Default)]
pub struct WaveInfo {
    pub zomie_count: u32,
    pub max_health: u32,
}

impl WaveInfo {
    pub fn new() -> Self {
        Self {
            zomie_count: 5,
            max_health: 50,
        }
    }
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

        for _ in 0..self.wave_info.zomie_count {
            let _ = self
                .spawn_zombie(
                    self.choose_position(&level_constrains),
                    self.wave_info.max_health,
                )
                .await;
        }

        self.wave_info.max_health += 10;
        self.wave_info.zomie_count += 2;
    }

    fn choose_position(&self, level_constrains: &(Vec2, Vec2)) -> Position {
        loop {
            let pos = Position {
                x: random_range(level_constrains.0.x..(level_constrains.1.x - 32.0)),
                y: random_range(level_constrains.0.y..(level_constrains.1.y - 32.0)),
            };

            if !is_occupied(&self.state.world, &pos) {
                return pos;
            }
        }
    }
}
