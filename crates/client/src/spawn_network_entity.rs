use game::ecs::{entities::tile::Tile, transform::Position};
use hecs::{Entity, World};
use protocol::spawn_entity::PacketSpawnEntity;

pub fn spawn_network_entity(world: &mut World, packet: PacketSpawnEntity) -> Entity {
    use protocol::spawn_entity::EntityKind;
    match packet.kind {
        EntityKind::Tile => Tile::spawn(world, Position::from(packet.pos), packet.network_id),
        EntityKind::Player => todo!(),
    }
}
