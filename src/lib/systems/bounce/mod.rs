use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use crate::lib::utils::rect::*;
use crate::lib::ball::*;
use crate::lib::paddle::*;
use crate::lib::arena::*;
use crate::lib::side::*;

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let r = ball.radius;
            let (vx, vy) = ball.velocity;
            let ball_x = transform.translation().x.as_f32();
            let ball_y = transform.translation().y.as_f32();

            // Bounce at the top or bottom of the arena
            if (ball_y <= r && vy < 0.0) || (ball_y >= ARENA_HEIGHT - r && vy > 0.0) {
                ball.reverse_vy();
            }

            // Bounce at paddle
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let translation = paddle_transform.translation();
                let paddle_x = translation.x.as_f32() - (paddle.width * 0.5);
                let paddle_y = translation.y.as_f32() - (paddle.height * 0.5);

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its center is within the larger wrapper
                // rectangle.
                let collision_area = Rect(
                    paddle_x - r,
                    paddle_y - r,
                    paddle_x + paddle.width + r,
                    paddle_y + paddle.height + r,
                );
                if collision_area.has_point(ball_x, ball_y) {
                    use Side::*;
                    if (paddle.side == Left && vx < 0.0) || (paddle.side == Right && vx > 0.0) {
                        ball.reverse_vx();
                    }
                };
            }
        }
    }
}
