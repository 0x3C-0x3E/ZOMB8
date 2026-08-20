use game::ecs::{entities::tile::Tile, transform::Position};
use hecs::{Entity, World};
use protocol::spawn_entity::SpawnEntity;

pub fn spawn_network_entity(world: &mut World, message: SpawnEntity) -> Entity {
    use protocol::spawn_entity::EntityKind;
    match message.kind {
        EntityKind::Tile => Tile::spawn(world, Position::from(message.pos), message.network_id),
        EntityKind::Player => todo!(),
    }
}
