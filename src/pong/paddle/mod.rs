use amethyst::ecs::*;
use super::side::Side;

pub const PADDLE_BLOCK: f32 = 4.0;
pub const PADDLE_WIDTH: f32 = PADDLE_BLOCK;
pub const PADDLE_HEIGHT: f32 = 4.0 * PADDLE_BLOCK;

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Self {
        let width = PADDLE_WIDTH;
        let height = PADDLE_HEIGHT;
        Paddle { side, width, height }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
