use amethyst::prelude::*;
use super::fns::*;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        initialize_paddles(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}
