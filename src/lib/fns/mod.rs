use std::path::*;
use std::io::Error;
use amethyst::core;
use amethyst::prelude::*;
use amethyst::renderer;
use amethyst::assets;
use amethyst::ui;
use crate::lib::components::paddle::*;
use crate::lib::arena::*;
use crate::lib::side::*;
use crate::lib::components::ball::*;
use crate::lib::score::*;

pub fn app_root() -> Result<PathBuf, Error> {
    amethyst::utils::application_root_dir()
}

fn get_path(filename: &str) -> Result<String, Option<Error>> {
    app_root()
        .map_err(Some)?
        .join(filename)
        .to_str()
        .map(|x| x.to_owned())
        .ok_or(None)
}

pub fn initialize_camera(world: &mut World) {
    let mut transform = core::Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(renderer::Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

pub fn load_sprite_sheet(world: &World) -> assets::Handle<renderer::SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<assets::Loader>();
        let texture_storage = world.read_resource::<assets::AssetStorage<renderer::Texture>>();
        loader.load(
            get_path("assets/texture/pong_spritesheet.png").unwrap(),
            renderer::ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<assets::Loader>();
    let sprite_sheet_store = world.read_resource::<assets::AssetStorage<renderer::SpriteSheet>>();
    loader.load(
        get_path("assets/texture/pong_spritesheet.ron").unwrap(),
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

pub fn initialize_ball(world: &mut World, sprite_sheet_handle: assets::Handle<renderer::SpriteSheet>) {
    let mut local_transform = core::Transform::default();
    local_transform.set_translation_xyz(
        ARENA_WIDTH / 2.0,
        ARENA_HEIGHT / 2.0,
        0.0,
    );

    let sprite_render = renderer::SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: (BALL_VELOCITY_X, BALL_VELOCITY_Y),
        })
        .with(local_transform)
        .build();
}

pub fn initialize_scoreboard(world: &mut World) {
    let mut create_score = |label: &str, x: f32| {
        let font = world.read_resource::<assets::Loader>().load(
            get_path("assets/font/square.ttf").unwrap(),
            ui::TtfFormat,
            (),
            &world.read_resource(),
        );

        let player_transform = ui::UiTransform::new(
            label.to_owned(),
            ui::Anchor::TopMiddle,
            ui::Anchor::TopMiddle,
            x,
            -50.0,
            -1.0,
            -200.0,
            -50.0,
        );

        world
            .create_entity()
            .with(player_transform)
            .with(ui::UiText::new(
                font,
                "0".to_owned(),
                [1.0, 1.0, 1.0, 1.0],
                50.0,
            ))
            .build()
    };

    let player1 = create_score("P1", -50.0);
    let player2 = create_score("P2", 50.0);
    world.add_resource(ScoreText { player1, player2 });
}
