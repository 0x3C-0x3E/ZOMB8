use hecs::{Entity, World};
use macroquad::prelude::*;

use crate::ecs::{
    components::{moveable::Moveable, transform::Position},
    entities::player::Player,
};

pub enum Axis {
    X,
    Y,
}

fn colliding(p1: &Position, p2: &Position) -> bool {
    p1.x < p2.x + 8.0 && p1.x + 8.0 > p2.x && p1.y < p2.y + 8.0 && p1.y + 8.0 > p2.y
}

pub fn resolve_collision_system(world: &World, axis: Axis) {
    for _ in 0..3 {
        let mut collisions: Vec<(Entity, Entity)> = Vec::new();

        for (e1, p1) in world
            .query::<(Entity, &Position)>()
            .with::<&Moveable>()
            .iter()
        {
            for (e2, p2) in world.query::<(Entity, &Position)>().iter() {
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

            match axis {
                Axis::X => {
                    let overlap_left = (p1.x + 8.0) - p2.x;
                    let overlap_right = (p2.x + 8.0) - p1.x;
                    let hit_right = overlap_left < overlap_right;
                    if hit_right {
                        p1.x = p2.x - 8.0;
                    } else {
                        p1.x = p2.x + 8.0;
                    }
                }
                Axis::Y => {
                    let overlap_top = (p1.y + 8.0) - p2.y;
                    let overlap_bottom = (p2.y + 8.0) - p1.y;
                    let hit_top = overlap_top < overlap_bottom;
                    if hit_top {
                        p1.y = p2.y - 8.0;
                    } else {
                        p1.y = p2.y + 8.0;
                    }
                }
            }
        }
    }
}
