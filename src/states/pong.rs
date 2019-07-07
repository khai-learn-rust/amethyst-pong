use amethyst::core;
use amethyst::prelude::*;
use amethyst::renderer;

const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;

fn initialize_camera(world: &mut World) {
    let mut transform = core::Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(renderer::Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        initialize_camera(world);
    }
}
