use amethyst::core;
use amethyst::prelude::*;
use amethyst::renderer;
use amethyst::ecs::*;

const ARENA_WIDTH: f32 = 100.0;
const ARENA_HEIGHT: f32 = 100.0;

fn initialize_camera(world: &mut World) {
    let mut transform = core::Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(renderer::Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

const PADDLE_BLOCK: f32 = 4.0;
const PADDLE_WIDTH: f32 = PADDLE_BLOCK;
const PADDLE_HEIGHT: f32 = 4.0 * PADDLE_BLOCK;

pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Self {
        let width = PADDLE_WIDTH;
        let height = PADDLE_HEIGHT;
        Paddle { side, width, height }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_paddle(mut world: &mut World) {
    use amethyst::core::Transform;

    let y = ARENA_HEIGHT / 2.0;

    let create_plank = |world: &mut World, side, x| {
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.0);

        world
            .create_entity()
            .with(Paddle::new(side))
            .with(transform)
            .build();
    };

    create_plank(&mut world, Side::Left, PADDLE_WIDTH * 0.5);
    create_plank(&mut world, Side::Right, PADDLE_WIDTH - PADDLE_HEIGHT * 0.5);
}

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        world.register::<Paddle>();
        initialize_camera(world);
        initialize_paddle(world);
    }
}
