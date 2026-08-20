use game::ecs::entities::tile::Tile;
use hecs::{Entity, World};
use protocol::spawn_entity::SpawnEntity;

pub fn spawn_network_entity(world: &mut World, message: SpawnEntity) -> Entity {
    use protocol::spawn_entity::EntityKind;
    match message.kind {
        EntityKind::Tile => Tile::spawn(world, message.pos.into(), message.network_id),
        EntityKind::Player => todo!(),
    }
}
