use amethyst::ecs::*;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Ball {
    pub velocity: (f32, f32),
    pub radius: f32,
}

impl Ball {
    /// Reverse horizontal velocity
    pub fn reverse_vx(&mut self) {
        self.velocity.0 = -self.velocity.0;
    }

    /// Reverse vertical velocity
    pub fn reverse_vy(&mut self) {
        self.velocity.1 = -self.velocity.1;
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
