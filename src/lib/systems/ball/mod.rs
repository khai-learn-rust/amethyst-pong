use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::*;
use crate::lib::components::ball::*;

pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
        for (ball, local) in (&balls, &mut locals).join() {
            let (vx, vy) = ball.velocity;
            let dt = time.delta_seconds();
            local.prepend_translation_x(vx * dt);
            local.prepend_translation_y(vy * dt);
        }
    }
}
