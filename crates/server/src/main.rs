use std::time::Duration;

use game::ecs::systems::physics::physics_system;
use protocol::TPS;

use crate::server::Server;

mod network_id_allocator;
mod network_thread;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server = Server::new().await?;

    loop {
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
            let _ = server.handle_packet(packet);
        }

        let dt = 1.0 / TPS as f32;
        physics_system(&mut server.state, dt);

        server.send_snapshot().await;

        let _ = server.check_for_disconnects().await;
        server.tick += 1;
        tokio::time::sleep(Duration::from_secs_f64(1.0 / TPS as f64)).await;
    }
}
