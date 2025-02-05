pub struct MotivationSystem {
    pub curiosity_factor: f32,
    pub exploration_drive: f32,
    pub novelty_seeking: f32,
}

impl MotivationSystem {
    pub fn calculate_interest(&self, stimulus: &Stimulus) -> f32 {
        let base_interest = self.curiosity_factor * stimulus.novelty;
        base_interest.powf(self.exploration_drive)
    }
}
