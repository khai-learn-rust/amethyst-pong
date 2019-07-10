use amethyst::ecs::prelude::*;

pub type Score = u8;
pub const MAX_SCORE: Score = std::u8::MAX;

#[derive(Default)]
pub struct ScoreBoard {
    pub left: Score,
    pub right: Score,
}

pub struct ScoreText {
    pub player1: Entity,
    pub player2: Entity,
}
