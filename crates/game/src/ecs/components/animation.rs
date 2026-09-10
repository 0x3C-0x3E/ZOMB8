use macroquad::prelude::*;

use std::{any::Any, fmt::Debug};
pub trait AnimState: Debug + Send + Sync {
    fn get_y_pos(&self) -> u32;
    fn get_max_frame(&self) -> usize;

    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct AnimController {
    pub tick: f32,
    pub frame: usize,

    pub state: Box<dyn AnimState>,
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub enum PlayerAnimState {
    #[default]
    Idle,
    Run,
}

impl AnimState for PlayerAnimState {
    fn get_y_pos(&self) -> u32 {
        match self {
            Self::Idle => 0,
            Self::Run => 0,
        }
    }

    fn get_max_frame(&self) -> usize {
        match self {
            Self::Idle => 0,
            Self::Run => 3,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub enum ZombieAnimState {
    #[default]
    Spawn,
    Run,
}

impl AnimState for ZombieAnimState {
    fn get_y_pos(&self) -> u32 {
        match self {
            Self::Spawn => 8,
            Self::Run => 0,
        }
    }

    fn get_max_frame(&self) -> usize {
        match self {
            Self::Spawn => 3,
            Self::Run => 3,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
