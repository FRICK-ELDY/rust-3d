#[derive(Default)]
pub struct GameState {
    pub t: f32,
}

impl GameState {
    pub fn new() -> Self {
        Self { t: 0.0 }
    }
    pub fn update(&mut self, dt: f32) {
        self.t += dt;
    }
}
