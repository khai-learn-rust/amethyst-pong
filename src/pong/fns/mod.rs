use amethyst::core;
use amethyst::prelude::*;
use amethyst::renderer;
use amethyst::assets;
use super::paddle::*;
use super::arena::*;
use super::side::*;

pub fn initialize_camera(world: &mut World) {
    let mut transform = core::Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(renderer::Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

pub fn load_sprite_sheet(world: &mut World) -> assets::Handle<renderer::SpriteSheet> {
    let app_root = amethyst::utils::application_root_dir().unwrap();

    let texture_handle = {
        let loader = world.read_resource::<assets::Loader>();
        let texture_storage = world.read_resource::<assets::AssetStorage<renderer::Texture>>();
        loader.load(
            app_root.join("texture/pong_spritesheet.png").to_str().unwrap(),
            renderer::ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<assets::Loader>();
    let sprite_sheet_store = world.read_resource::<assets::AssetStorage<renderer::SpriteSheet>>();
    loader.load(
        app_root.join("texture/pong_spritesheet.ron").to_str().unwrap(),
        renderer::SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}

pub fn initialize_paddles(mut world: &mut World, sprite_sheet: assets::Handle<renderer::SpriteSheet>) {
    use amethyst::core::Transform;

    let y = ARENA_HEIGHT / 2.0;

    let create_plank = |world: &mut World, side, x| {
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.0);

        let sprite_render = renderer::SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(sprite_render)
            .with(Paddle::new(side))
            .with(transform)
            .build();
    };

    create_plank(&mut world, Side::Left, PADDLE_WIDTH * 0.5);
    create_plank(&mut world, Side::Right, ARENA_WIDTH - PADDLE_WIDTH * 0.5);
}
