use amethyst::prelude::*;
use amethyst::window::*;
use amethyst::renderer;
use amethyst::assets;
use amethyst::core;
use renderer::sprite;

mod pong;
use pong::Pong;

mod graph;
use graph::example_graph::ExampleGraph;

fn main() -> amethyst::Result<()> {
    let app_root = amethyst::utils::application_root_dir()?;
    let display_config_path = app_root.join("resources").join("display_config.ron");

    amethyst::start_logger(amethyst::LoggerConfig {
        stdout: amethyst::StdoutLog::Off,
        level_filter: amethyst::LogLevelFilter::Info,
        log_file: Some(app_root.join("pong.log")),
        allow_env_override: true,
        log_gfx_device_level: Some(amethyst::LogLevelFilter::Info),
    });

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with_bundle(core::transform::TransformBundle::new())?
        .with(
            assets::Processor::<sprite::SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with_thread_local(
            renderer::RenderingSystem::<renderer::types::DefaultBackend, _>::new(
                ExampleGraph::default(),
            ),
        );

    let app_root = std::path::PathBuf::from(".");
    let assets_dir = app_root.join("assets/");

    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    Ok(())
}
