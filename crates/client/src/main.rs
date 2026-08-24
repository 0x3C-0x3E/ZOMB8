#![allow(clippy::new_without_default)]

use crate::network_thread::client_network_loop;
use crate::{client::Client, snapshot_handler::FIXED_DT};
use game::ecs::systems::physics::physics_system_for_entity;
use game::{
    ecs::systems::{
        input::{get_input_map, input_system},
        rendering::rendering_system,
    },
    game::texture_manager::TextureManager,
};
use macroquad::prelude::*;
use protocol::{
    TPS,
    network_id::ProtocolNetworkId,
    packet::Packet,
    packets::{
        input::{InputMap, PacketInput},
        ping::PacketPing,
    },
};

mod client;
mod network_thread;
mod snapshot_handler;
mod spawn_despawn_handler;

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

    let (out_send, mut out_recv) = tokio::sync::mpsc::channel::<Packet>(100);
    let (in_send, in_recv) = tokio::sync::mpsc::channel::<Packet>(100);

    let (input_send, input_recv) = tokio::sync::watch::channel::<Packet>(Packet::from_payload(
        PacketInput::new(ProtocolNetworkId(0), 0, InputMap::zero()),
    )?);

    let mut client = Client::new(input_send);

    let mut texture_manager = TextureManager::new();

    texture_manager
        .load_texture("assets/img/tileset.png", "tileset")
        .await;

    texture_manager
        .load_texture("assets/img/player.png", "player")
        .await;

    texture_manager
        .load_texture("assets/img/mole.png", "mole")
        .await;

    let network_thread = std::thread::spawn(move || -> anyhow::Result<()> {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(client_network_loop(out_send, in_recv, input_recv))?;
        Ok(())
    });

    let payload = PacketPing::new();
    if let Ok(packet) = Packet::from_payload(payload) {
        let _ = in_send.send(packet).await;
    }

    let mut accumulator = 0.0f32;

    loop {
        if network_thread.is_finished() {
            panic!("network thread exited with {:?}", network_thread.join());
        }

        while let Ok(packet) = out_recv.try_recv() {
            let _ = client.handle_packet(packet);
        }

        accumulator += get_frame_time();
        while accumulator >= FIXED_DT {
            client.set_local_prev_pos();

            let input_map = get_input_map();
            client.check_for_new_input(&input_map)?;
            input_system(&mut client.state, client.client_id, &input_map);

            if let Some((pos, vel)) = client.get_player_state() {
                physics_system_for_entity(pos, vel, FIXED_DT);
            }
            accumulator -= FIXED_DT;
        }

        client.interp_timer += get_frame_time();
        let interp_alpha = (client.interp_timer / (1.0 / TPS as f32)).clamp(0.0, 1.0);

        client.set_local_render_pos(accumulator / FIXED_DT);
        client.set_net_render_pos(interp_alpha);

        rendering_system(&mut client.state, &texture_manager);
        next_frame().await;
    }
}
