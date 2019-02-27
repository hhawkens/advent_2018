use std::collections::VecDeque;

pub type Marble = usize;
pub type Circle = VecDeque<Marble>;
pub type Score = usize;

pub struct Player {
    pub id: usize,
    pub points: Score
}

#[derive(Debug)]
pub struct GameRules {
    pub num_players: usize,
    pub last_marble_points: Score
}
