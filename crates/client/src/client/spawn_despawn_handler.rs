use game::ecs::{
    entities::{bullet::Bullet, player::Player, tile::Tile, zombie::Zombie},
    network_id::NetworkId,
    transform::Position,
};
use glam::Vec2;
use hecs::{Entity, World};
use protocol::packets::{
    despawn_entity::PacketDespawnEntity, snapshot::EntityState, spawn_entity::PacketSpawnEntity,
};

pub fn spawn_network_entity_from_state(
    world: &mut World,
    state: EntityState,
    network_id: NetworkId,
) -> Entity {
    use protocol::packets::spawn_entity::EntityKind;
    match state.kind {
        EntityKind::Tile => Tile::spawn(world, Position::from(state.pos), network_id),
        EntityKind::Player => Player::spawn(world, Position::from(state.pos), network_id),
        EntityKind::Zombie => Zombie::spawn(world, Position::from(state.pos), network_id),
        EntityKind::Bullet => Bullet::spawn(world, Position::from(state.pos), 0.0, network_id),
    }
}

pub fn spawn_network_entity(world: &mut World, packet: PacketSpawnEntity) -> Entity {
    let state = EntityState {
        kind: packet.kind,
        pos: packet.pos,
        vel: Vec2::ZERO,
    };
    spawn_network_entity_from_state(world, state, packet.network_id)
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
