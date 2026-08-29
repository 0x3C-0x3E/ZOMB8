use game::ecs::{
    entities::{bullet::Bullet, player::Player, tile::Tile, zombie::Zombie},
    network_id::NetworkId,
    transform::Position,
};
use glam::Vec2;
use hecs::Entity;
use protocol::packets::{
    despawn_entity::PacketDespawnEntity, snapshot::EntityState, spawn_entity::PacketSpawnEntity,
};

use crate::client::core::Client;

impl Client {
    pub fn spawn_network_entity_from_state(
        &mut self,
        state: EntityState,
        network_id: NetworkId,
    ) -> Entity {
        use protocol::packets::spawn_entity::EntityKind;
        match state.kind {
            EntityKind::Tile => {
                Tile::spawn(&mut self.state.world, Position::from(state.pos), network_id)
            }
            EntityKind::Player => {
                Player::spawn(&mut self.state.world, Position::from(state.pos), network_id)
            }
            EntityKind::Zombie => {
                Zombie::spawn(&mut self.state.world, Position::from(state.pos), network_id)
            }
            EntityKind::Bullet => Bullet::spawn(
                &mut self.state.world,
                Position::from(state.pos),
                None,
                network_id,
            ),
        }
    }

    pub fn spawn_network_entity(&mut self, packet: PacketSpawnEntity) -> Entity {
        let state = EntityState {
            kind: packet.kind,
            pos: packet.pos,
            vel: Vec2::ZERO,
        };
        self.spawn_network_entity_from_state(state, packet.network_id)
    }

    pub fn despawn_network_entity(&mut self, packet: PacketDespawnEntity) {
        let found = self
            .state
            .world
            .query::<(Entity, &NetworkId)>()
            .into_iter()
            .find(|(_, id)| **id == packet.network_id)
            .map(|(e, _)| e);

        if let Some(e) = found {
            let _ = self.state.world.despawn(e);
        }
    }
}
