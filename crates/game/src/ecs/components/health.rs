pub struct Health(pub u32);

impl Health {
    pub fn get(&self) -> u32 {
        self.0
    }
}
