use game::ecs::{
    entities::{
        bullet::Bullet,
        particle::{Particle, ParticleKind},
        player::Player,
        tile::Tile,
        zombie::Zombie,
    },
    network_id::NetworkId,
    transform::Position,
};
use glam::Vec2;
use hecs::Entity;
use protocol::{
    network_id::ProtocolNetworkId,
    packets::{
        despawn_entity::PacketDespawnEntity,
        snapshot::EntityState,
        spawn_entity::{EntityKind, PacketSpawnEntity},
    },
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
            EntityKind::Zombie => Zombie::spawn(
                &mut self.state.world,
                Position::from(state.pos),
                network_id,
                None,
            ),
            EntityKind::Bullet => Bullet::spawn(
                &mut self.state.world,
                Position::from(state.pos),
                None,
                network_id,
                ProtocolNetworkId(0),
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
            .query::<(Entity, &EntityKind, &NetworkId)>()
            .into_iter()
            .find(|(_, _, id)| **id == packet.network_id)
            .map(|(e, kind, _)| (e, *kind));

        if let Some((e, kind)) = found {
            use protocol::packets::spawn_entity::EntityKind;
            match kind {
                EntityKind::Player => {
                    self.spawn_particles(e, ParticleKind::DeathPlayer);
                }
                EntityKind::Zombie => {
                    self.spawn_particles(e, ParticleKind::DeathZombie);
                }
                EntityKind::Bullet => {
                    self.spawn_particles(e, ParticleKind::BulletCollision);
                }
                _ => {}
            }

            if Some(e) == self.player {
                self.player = None;
                self.client_id = ProtocolNetworkId(0);
            }

            let _ = self.state.world.despawn(e);
        }
    }

    pub fn spawn_particles(&mut self, entity: Entity, kind: ParticleKind) {
        let Ok(entity_pos) = self.state.world.get::<&Position>(entity) else {
            panic!("particle system cannot spawn on entity without position");
        };

        let pos = *entity_pos;
        drop(entity_pos);

        let particle_count = match kind {
            ParticleKind::DeathZombie => 5,
            ParticleKind::DeathPlayer => 10,
            ParticleKind::BulletCollision => 3,
            ParticleKind::BulletTrail => 2,
        };

        for _ in 0..particle_count {
            Particle::spawn(&mut self.state.world, pos, &kind);
        }
    }
}
