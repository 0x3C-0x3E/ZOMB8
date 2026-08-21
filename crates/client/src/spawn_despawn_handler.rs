use game::ecs::{
    entities::{player::Player, tile::Tile},
    network_id::NetworkId,
    transform::Position,
};
use hecs::{Entity, World};
use protocol::packets::{despawn_entity::PacketDespawnEntity, spawn_entity::PacketSpawnEntity};

pub fn spawn_network_entity(world: &mut World, packet: PacketSpawnEntity) -> Entity {
    use protocol::packets::spawn_entity::EntityKind;
    match packet.kind {
        EntityKind::Tile => Tile::spawn(world, Position::from(packet.pos), packet.network_id),
        EntityKind::Player => Player::spawn(world, Position::from(packet.pos), packet.network_id),
    }
}

pub fn despawn_network_entity(world: &mut World, packet: PacketDespawnEntity) {
    let found = world
        .query::<(Entity, &NetworkId)>()
        .into_iter()
        .find(|(_, id)| **id == packet.network_id)
        .map(|(e, _)| e);

    if let Some(e) = found {
        let _ = world.despawn(e);
    }
}
