use game::{
    ecs::{entities::tile::Tile, transform::Position},
    game::state::State,
};

use crate::network_id_allocator::NetworkIdAllocator;

mod network_id_allocator;

#[tokio::main]
async fn main() {
    let mut allocator = NetworkIdAllocator::new();

    let mut state = State::new();
    let _ = Tile::spawn(&mut state.world, Position::zero(), allocator.allocate());
}
