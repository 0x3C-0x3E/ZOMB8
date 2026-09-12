use hecs::{Entity, World};
use protocol::packets::spawn_entity::EntityKind;

use crate::ecs::{
    components::{health::Health, moveable::CollisionMesh},
    entities::{health_pack::HealthPack, player::Player},
    network_id::NetworkId,
    systems::zombie_movement::get_closest_entity,
    transform::Position,
};

pub fn health_pack_update_system(world: &mut World) -> Vec<(Entity, NetworkId)> {
    let mut packs_to_remove = Vec::new();
    for (e, pos, mesh, id) in world
        .query::<(Entity, &Position, &CollisionMesh, &NetworkId)>()
        .with::<&HealthPack>()
        .iter()
    {
        if mesh.any_of_kind(EntityKind::Player) {
            let Some(player) = get_closest_entity::<&Player>(world, pos) else {
                continue;
            };
            let mut health = world.get::<&mut Health>(player).unwrap();
            health.health = health.health.saturating_add(50).clamp(0, health.max);
            packs_to_remove.push((e, *id));
        }
    }
    packs_to_remove
}
