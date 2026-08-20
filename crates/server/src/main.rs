use crate::server::TPS;
use std::{thread::sleep, time::Duration};

use game::ecs::systems::{input::input_system, physics::physics_system};

use crate::server::Server;

mod network_id_allocator;
mod network_thread;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server = Server::new().await?;

    loop {
        if server.network_thread.is_finished() {
            panic!(
                "network thread exited with {:?}",
                server.network_thread.await?
            );
        }
        while let Ok((sender_addr, packet)) = server.out_recv.try_recv() {
            if server.check_insert_client(sender_addr) {
                let _ = server.create_new_player(sender_addr).await;
            }

            server.check_for_disconnects();

            server.touch_client(sender_addr);
            server.handle_packet(packet);
        }

        physics_system(&mut server.state, 1.0 / TPS as f32);

        server.send_snapshot().await;

        server.tick += 1;
        sleep(Duration::from_millis((1000 / TPS).into()));
    }
}
