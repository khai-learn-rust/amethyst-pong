use amethyst::prelude::*;
use amethyst::renderer::*;
use amethyst::assets::*;
use amethyst::core::timing::Time;
use super::fns::*;

#[derive(Debug, Default)]
pub struct Pong {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        self.ball_spawn_timer.replace(1.0);
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        initialize_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        initialize_camera(world);
        initialize_scoreboard(world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            {
                let time = data.world.res.fetch::<Time>();
                timer -= time.delta_seconds();
            }

            if timer <= 0.0 {
                initialize_ball(
                    data.world,
                    self.sprite_sheet_handle.clone().unwrap()
                );
            } else {
                self.ball_spawn_timer.replace(timer);
            }
        }

        Trans::None
    }
}
