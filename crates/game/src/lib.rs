#![allow(clippy::new_without_default)]
pub mod ecs;
pub mod game;

#[cfg(all(feature = "server", feature = "client"))]
compile_error!("feature server and feature client cannot be enabled at the same time");
