use macroquad::prelude::*;

use std::{any::Any, fmt::Debug};
pub trait AnimState: Debug + Send + Sync {
    fn get_id_name(&self) -> &str;
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
    fn get_id_name(&self) -> &str {
        match self {
            Self::Idle => "player",
            Self::Run => "player",
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
    Idle,
    Run,
}

impl AnimState for ZombieAnimState {
    fn get_id_name(&self) -> &str {
        match self {
            Self::Idle => "zombie",
            Self::Run => "zombie",
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
