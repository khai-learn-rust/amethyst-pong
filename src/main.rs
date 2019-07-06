use amethyst::prelude::*;
use amethyst::window::*;
use amethyst::renderer;
use amethyst::ecs;
use amethyst::assets;
use renderer::rendy;
use renderer::sprite;

struct Pong;

impl SimpleState for Pong {}

#[derive(Default)]
struct ExampleGraph {
    dimensions: Option<ScreenDimensions>,
    dirty: bool,
}

impl renderer::GraphCreator<renderer::types::DefaultBackend> for ExampleGraph {
    fn rebuild(&mut self, res: &ecs::Resources) -> bool {
        use std::ops::Deref;

        let new_dim = res.try_fetch::<ScreenDimensions>();
        if self.dimensions.as_ref() != new_dim.as_ref().map(Deref::deref) {
            self.dirty = true;
            self.dimensions = new_dim.map(|x| x.clone());
            return false;
        }

        return self.dirty;
    }

    fn builder(
        &mut self,
        factory: &mut renderer::Factory<renderer::types::DefaultBackend>,
        res: &ecs::Resources,
    ) -> renderer::GraphBuilder<renderer::types::DefaultBackend, ecs::Resources> {
        use rendy::graph::present::PresentNode;
        use rendy::hal::command::{ClearDepthStencil, ClearValue};
        use renderer::RenderGroupDesc;

        self.dirty = false;

        // Retrieve a reference to the target window, which is created by the WindowBundle
        let window = res.fetch();
        let dims = self.dimensions.as_ref().expect("Failed to get dimensions");
        let win_kind = renderer::Kind::D2(
            dims.width() as u32,
            dims.height() as u32,
            1,
            1,
        );

        // Create a new drawing surface
        let surface = factory.create_surface(&window);
        let surface_format = factory.get_surface_format(&surface);

        // Begin building RenderGraph
        let mut graph_builder = renderer::GraphBuilder::new();

        let color = graph_builder.create_image(
            win_kind,
            1,
            surface_format,
            Some(ClearValue::Color([0.0, 0.0, 0.0, 1.0].into())), // black
        );

        let depth = graph_builder.create_image(
            win_kind,
            1,
            renderer::Format::D32Sfloat,
            Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        );

        let pass = graph_builder.add_node(
            renderer::SubpassBuilder::new()
                .with_group(RenderGroupDesc::builder(renderer::pass::DrawFlat2DDesc::new()))
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        graph_builder.add_node(
            PresentNode::builder(factory, surface, color)
                .with_dependency(pass),
        );

        graph_builder
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = amethyst::utils::application_root_dir()?;
    let display_config_path = app_root.join("resources").join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
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
