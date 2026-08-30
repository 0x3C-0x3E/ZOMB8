pub struct Score(pub u32);

impl Score {
    pub fn get(&self) -> u32 {
        self.0
    }
}
