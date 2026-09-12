use hecs::World;
use protocol::{packet::Packet, packets::wave::PacketWave};
use rand::random_range;

use game::ecs::{
    entities::{tile::Tile, zombie::Zombie},
    systems::pathfinding::GridPos,
    transform::Position,
};

use crate::server::Server;

fn is_occupied(world: &World, pos: &GridPos) -> bool {
    world
        .query::<&Position>()
        .with::<&Tile>()
        .into_iter()
        .find(|t_pos| Into::<GridPos>::into(*t_pos) == *pos)
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
            zomie_count: 1,
            max_health: 50,
        }
    }
}

impl Server {
    pub async fn spawn_wave_system(&mut self) {
        let zombies_count = self.state.world.query::<&Zombie>().into_iter().len();
        if zombies_count == 0 {
            self.state.current_wave += 1;
            self.spawn_wave().await;
        }
    }

    async fn spawn_wave(&mut self) {
        let level_constrains = self.server_data.level_constraints;

        let payload = PacketWave::new(self.state.current_wave);
        let packet = Packet::from_payload(payload).unwrap();

        let _ = self.send_to_all(&packet).await;

        for _ in 0..self.server_data.wave_info.zomie_count {
            let _ = self
                .spawn_zombie(
                    self.choose_position(&level_constrains).into(),
                    self.server_data.wave_info.max_health,
                )
                .await;
        }

        let _ = self
            .spawn_health_pack(self.choose_position(&level_constrains).into())
            .await;

        self.server_data.wave_info.max_health += 10;
        self.server_data.wave_info.zomie_count += 2;
    }

    fn choose_position(&self, cons: &(GridPos, GridPos)) -> GridPos {
        loop {
            let pos = GridPos {
                x: random_range(cons.0.x..cons.1.x),
                y: random_range(cons.0.y..cons.1.y),
            };

            if !is_occupied(&self.state.world, &pos) {
                return pos;
            }
        }
    }

    pub fn get_level_constraints(world: &World) -> (GridPos, GridPos) {
        let min: Option<GridPos> = world
            .query::<&Position>()
            .with::<&Tile>()
            .iter()
            .min_by_key(|p| {
                let pos: GridPos = (*p).into();
                pos
            })
            .map(|p| p.into());

        let max: Option<GridPos> = world
            .query::<&Position>()
            .with::<&Tile>()
            .iter()
            .max_by_key(|p| {
                let pos: GridPos = (*p).into();
                pos
            })
            .map(|p| p.into());

        (min.unwrap(), max.unwrap())
    }
}
