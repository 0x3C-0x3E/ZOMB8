use std::time::Duration;

use game::ecs::systems::{
    bullet_movement::bullet_movement_system,
    pathfinding::zombie_pathfinding_system,
    physics::physics_system,
    timers::zombie_update_timers,
    zombie_movement::{zombie_collision_system, zombie_movement_system},
};
use protocol::config_parser::{parse_config, tps};
use tokio::time::Instant;

use crate::server::Server;

mod network_id_allocator;
mod network_thread;
mod packet_handler;
mod server;
mod wave;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    parse_config().unwrap();

    let mut server = Server::new().await?;

    let dt = 1.0 / tps() as f32;

    loop {
        let tick_begin = Instant::now();

        if server.network_thread.is_finished() {
            match server.network_thread.await {
                Ok(res) => panic!("network thread exited with {:?}", res),
                Err(e) => panic!("network thread exited with {:?}", e),
            }
        }

        while let Ok((sender_addr, packet)) = server.out_recv.try_recv() {
            if server.check_insert_client(sender_addr) {
                let _ = server.create_new_player(sender_addr).await;
            }

            server.touch_client(sender_addr);
            let _ = server.handle_packet(sender_addr, packet).await;
        }

        server.spawn_wave_system().await;

        let mut remove = zombie_collision_system(&mut server.state.world);
        remove.extend(bullet_movement_system(&mut server.state.world));
        for (entity, id) in remove {
            let _ = server.despawn_entity(entity, id).await;
        }

        zombie_pathfinding_system(
            &mut server.state.world,
            &server.server_data.tile_grid,
            &server.server_data.level_constraints,
            &mut server.server_data.zombie_paths,
        );

        zombie_update_timers(&mut server.state.world, dt);

        zombie_movement_system(
            &mut server.state.world,
            &mut server.server_data.zombie_paths,
            dt,
        );

        physics_system(&server.state.world, dt);

        server.check_player_health().await;
        server.send_snapshot().await;

        let _ = server.check_for_disconnects().await;
        server.tick += 1;
        tokio::time::sleep_until(tick_begin + Duration::from_secs_f64(1.0 / tps() as f64)).await;
    }
}
