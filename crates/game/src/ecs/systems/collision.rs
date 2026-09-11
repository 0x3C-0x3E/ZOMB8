use hecs::{Entity, World};
use macroquad::prelude::*;
use protocol::packets::spawn_entity::EntityKind;

use crate::ecs::components::{
    moveable::{Collideable, CollisionMesh, Moveable},
    transform::Position,
};

pub enum Axis {
    X,
    Y,
}

fn colliding(p1: &Position, p2: &Position) -> bool {
    p1.x < p2.x + 7.9 && p1.x + 7.9 > p2.x && p1.y < p2.y + 7.9 && p1.y + 7.9 > p2.y
}

pub fn resolve_collision_system(world: &World, axis: Axis) {
    for _ in 0..3 {
        let mut collisions: Vec<(Entity, Entity)> = Vec::new();

        for (e1, p1) in world
            .query::<(Entity, &Position)>()
            .with::<&Moveable>()
            .with::<&CollisionMesh>()
            .iter()
        {
            for (e2, p2) in world
                .query::<(Entity, &Position)>()
                .with::<&CollisionMesh>()
                .iter()
            {
                if e1 == e2 {
                    continue;
                }

                if colliding(p1, p2) && !collisions.contains(&(e2, e1)) {
                    collisions.push((e1, e2));
                }
            }
        }

        for (e1, e2) in collisions {
            let p2 = *world.entity(e2).unwrap().get::<&Position>().unwrap();
            let mut p1 = world.entity(e1).unwrap().get::<&mut Position>().unwrap();

            let k1 = *world.get::<&EntityKind>(e1).unwrap();
            let k2 = *world.get::<&EntityKind>(e2).unwrap();

            let mut collision_mesh = CollisionMesh::default();

            let is_collidiable =
                world.get::<&Collideable>(e1).is_ok() && world.get::<&Collideable>(e2).is_ok();

            match axis {
                Axis::X => {
                    let overlap_left = (p1.x + 8.0) - p2.x;
                    let overlap_right = (p2.x + 8.0) - p1.x;
                    let hit_right = overlap_left < overlap_right;
                    if hit_right {
                        collision_mesh.right = Some(k2);
                        if is_collidiable {
                            p1.x = p2.x - 8.0;
                        }
                    } else {
                        collision_mesh.left = Some(k2);
                        if is_collidiable {
                            p1.x = p2.x + 8.0;
                        }
                    }
                }
                Axis::Y => {
                    let overlap_top = (p1.y + 8.0) - p2.y;
                    let overlap_bottom = (p2.y + 8.0) - p1.y;
                    let hit_top = overlap_top < overlap_bottom;
                    if hit_top {
                        collision_mesh.top = Some(k2);
                        if is_collidiable {
                            p1.y = p2.y - 8.0;
                        }
                    } else {
                        collision_mesh.bottom = Some(k2);
                        if is_collidiable {
                            p1.y = p2.y + 8.0;
                        }
                    }
                }
            }

            {
                let Some(mut mesh) = world.entity(e1).unwrap().get::<&mut CollisionMesh>() else {
                    continue;
                };
                mesh.update(&collision_mesh);
            }
            let collision_mesh = CollisionMesh::invert(&collision_mesh, k1);
            {
                let Some(mut mesh) = world.entity(e2).unwrap().get::<&mut CollisionMesh>() else {
                    continue;
                };
                mesh.update(&collision_mesh);
            }
        }
    }
}
