use game::ecs::network_id::NetworkId;

pub struct NetworkIdAllocator {
    next: u32,
}

impl NetworkIdAllocator {
    pub fn new() -> Self {
        Self { next: 1 }
    }

    pub fn allocate(&mut self) -> NetworkId {
        let id = NetworkId(self.next);
        self.next += 1;
        id
    }
}
