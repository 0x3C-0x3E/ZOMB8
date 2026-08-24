use hecs::World;
use macroquad::prelude::*;

use crate::ecs::{
    components::{
        animation::{AnimController, PlayerAnimState, ZombieAnimState},
        sprite::Sprite,
        transform::Velocity,
    },
    entities::{player::Player, zombie::Zombie},
};

pub fn player_animation_state_system(world: &mut World) {
    for (vel, controller, sprite) in world
        .query_mut::<(&Velocity, &mut AnimController, &mut Sprite)>()
        .with::<&Player>()
    {
        let new_state = if vel.vec2().length() < 0.1 {
            PlayerAnimState::Idle
        } else {
            PlayerAnimState::Run
        };

        if let Some(current) = controller.state.as_any().downcast_ref::<PlayerAnimState>()
            && *current != new_state
        {
            controller.state = Box::new(new_state);
            controller.tick = 0.0;
            sprite.set_id(controller.state.get_id_name());
        }
    }
}

pub fn zombie_animation_state_system(world: &mut World) {
    for controller in world.query_mut::<&mut AnimController>().with::<&Zombie>() {
        controller.state = Box::new(ZombieAnimState::Run);
    }
}

pub fn animation_playback_system(world: &mut World) {
    for (controller, sprite) in world.query_mut::<(&mut AnimController, &mut Sprite)>() {
        controller.tick += 1.0 * get_frame_time().min(1.0 / 30.0);
        if controller.tick >= 0.09 {
            controller.tick = 0.0;
            controller.frame += 1;
            if controller.frame > controller.state.get_max_frame() {
                controller.frame = 0;
            }

            sprite.rect.x = controller.frame as f32 * sprite.rect.w;
        }
    }
}
