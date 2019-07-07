pub mod side;
pub mod arena;
pub mod paddle;
pub mod fns;

use amethyst::prelude::*;
use paddle::*;
use fns::*;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        world.register::<Paddle>();
        initialize_paddles(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}
