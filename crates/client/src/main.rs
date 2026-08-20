#![allow(clippy::new_without_default)]

use crate::client::Client;
use crate::network_thread::client_network_loop;
use game::{
    ecs::systems::rendering::rendering_system,
    game::{state::State, texture_manager::TextureManager},
};
use macroquad::prelude::*;
use protocol::packet::Packet;

mod client;
mod network_thread;
mod spawn_network_entity;

fn window_conf() -> Conf {
    Conf {
        window_title: "".to_owned(),
        window_width: 600.0 as i32,
        window_height: 600.0 as i32,
        window_resizable: true,
        sample_count: 1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> anyhow::Result<()> {
    set_default_filter_mode(FilterMode::Nearest);

    let mut client = Client::new();

    let mut texture_manager = TextureManager::new();

    texture_manager
        .load_texture("assets/img/tileset.png", "tileset")
        .await;

    texture_manager
        .load_texture("assets/img/player.png", "player")
        .await;

    let (out_send, mut out_recv) = tokio::sync::mpsc::channel::<Packet>(100);
    let (in_send, in_recv) = tokio::sync::mpsc::channel::<Packet>(100);

    let network_thread = std::thread::spawn(move || -> anyhow::Result<()> {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(client_network_loop(out_send, in_recv))?;
        Ok(())
    });

    loop {
        if network_thread.is_finished() {
            panic!("network thread exited with {:?}", network_thread.join());
        }

        while let Ok(packet) = out_recv.try_recv() {
            client.handle_packet(packet);
        }

        rendering_system(&mut client.state, &texture_manager);
        next_frame().await;
    }
}
