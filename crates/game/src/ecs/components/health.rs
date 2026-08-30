pub struct Health {
    pub health: u32,
    pub max: u32,
}

impl Health {
    pub fn new(init_health: u32, max: u32) -> Self {
        Self {
            health: init_health,
            max,
        }
    }

    pub fn get(&self) -> u32 {
        self.health
    }

    pub fn get_max(&self) -> u32 {
        self.max
    }
}
