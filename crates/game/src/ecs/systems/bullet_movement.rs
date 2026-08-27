use glam::Vec2;
use hecs::{Entity, World};
use protocol::packets::spawn_entity::EntityKind;

use crate::ecs::{
    components::{moveable::CollisionMesh, sprite::Rotation, transform::Velocity},
    entities::bullet::Bullet,
    network_id::NetworkId,
    transform::Position,
};

pub fn bullet_movement_system(world: &mut World) -> Vec<(Entity, NetworkId)> {
    let mut bullets_to_remove = Vec::new();
    for (e, pos, vel, rotation, id, mesh) in world
        .query_mut::<(
            Entity,
            &Position,
            &mut Velocity,
            &Rotation,
            &NetworkId,
            &CollisionMesh,
        )>()
        .with::<&Bullet>()
    {
        println!("{:?}", mesh);
        let mut vec_vel = Vec2::from_angle(rotation.0.to_radians());
        vec_vel *= 140.0;
        vel.update_vec2(vec_vel);

        if pos.vec2().distance_squared(Vec2::ZERO) > 500.0_f32.powi(2)
            || mesh.any_of_kind(EntityKind::Tile)
        {
            bullets_to_remove.push((e, *id));
        }
    }

    bullets_to_remove
}
