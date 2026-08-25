#![allow(clippy::new_without_default)]

use crate::network_thread::client_network_loop;
use crate::{client::core::Client, client::snapshot_handler::FIXED_DT};
use game::ecs::systems::animation::{
    animation_playback_system, player_animation_state_system, zombie_animation_state_system,
};
use game::ecs::systems::physics::physics_system_for_player;
use game::{
    ecs::systems::{
        input::{get_input_map, input_system},
        rendering::rendering_system,
    },
    game::texture_manager::TextureManager,
};
use macroquad::prelude::*;
use protocol::packets::request::{PacketRequest, RequestKind};
use protocol::{
    TPS,
    network_id::ProtocolNetworkId,
    packet::Packet,
    packets::input::{InputMap, PacketInput},
};

mod client;
mod network_thread;

fn window_conf() -> Conf {
    Conf {
        window_title: "ZOMB8".to_owned(),
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

    let texture_manager = TextureManager::load_game_textures().await;

    let (out_send, out_recv) = tokio::sync::mpsc::channel::<Packet>(100);
    let (in_send, in_recv) = tokio::sync::mpsc::channel::<Packet>(100);

    let (input_send, input_recv) = tokio::sync::watch::channel::<Packet>(Packet::from_payload(
        PacketInput::new(ProtocolNetworkId(0), 0, InputMap::zero()),
    )?);

    let mut client = Client::new(out_recv, in_send, input_send);

    let network_thread = std::thread::spawn(move || -> anyhow::Result<()> {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(client_network_loop(out_send, in_recv, input_recv))?;
        Ok(())
    });

    let payload = PacketRequest::new(RequestKind::PlayerId);
    if let Ok(packet) = Packet::from_payload(payload) {
        client.send(packet).await;
    }

    let payload = PacketRequest::new(RequestKind::LevelData);
    if let Ok(packet) = Packet::from_payload(payload) {
        client.send(packet).await;
    }

    let mut accumulator = 0.0f32;

    loop {
        if network_thread.is_finished() {
            panic!("network thread exited with {:?}", network_thread.join());
        }

        client.try_recv();
        client.input_system();

        accumulator += get_frame_time();
        while accumulator >= FIXED_DT {
            client.set_local_prev_pos();
            if let Some(player) = client.player {
                physics_system_for_player(&client.state.world, player, FIXED_DT);
            }
            accumulator -= FIXED_DT;
        }

        client.interp_timer += get_frame_time();
        let interp_alpha = (client.interp_timer / (1.0 / TPS as f32)).clamp(0.0, 1.0);

        client.set_local_render_pos(accumulator / FIXED_DT);
        client.set_net_render_pos(interp_alpha);

        player_animation_state_system(&mut client.state.world);
        zombie_animation_state_system(&mut client.state.world);

        animation_playback_system(&mut client.state.world);

        if let Some((pos, _)) = client.get_player_state() {
            let pos = *pos;
            client.state.rendering_state.set_camera(pos);
        }

        rendering_system(&mut client.state, &texture_manager);
        next_frame().await;
    }
}
