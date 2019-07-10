use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::*;
use amethyst::ui::UiText;
use crate::lib::components::ball::*;
use crate::lib::arena::*;
use crate::lib::score::*;

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, sys_data: Self::SystemData) {
        let (
            mut balls,
            mut locals,
            mut ui_text,
            mut scores,
            score_text,
        ) = sys_data;

        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_r = ball.radius;
            let ball_x = transform.translation().x.as_f32();

            type TargetScore = FnOnce(&mut ScoreBoard) -> &mut Score;
            let mut act_on_hit = |target_score: Box<TargetScore>, target_entity: Entity| {
                let score = {
                    let reference = target_score(&mut scores);
                    *reference = (*reference + 1).min(MAX_SCORE);
                    *reference
                };

                if let Some(text) = ui_text.get_mut(target_entity) {
                    text.text = score.to_string();
                }

                ball.reverse_vx();
                transform.set_translation_x(ARENA_WIDTH / 2.0); // reset position

                println!(
                    "Score: | {:^3} | {:^3} |",
                    scores.left, scores.right,
                );
            };

            if ball_x <= ball_r {
                act_on_hit(
                    Box::new(|scores| &mut scores.right),
                    score_text.player2
                );
            } else if ball_x >= ARENA_WIDTH - ball_r {
                act_on_hit(
                    Box::new(|scores| &mut scores.left),
                    score_text.player1
                );
            }
        }
    }
}
