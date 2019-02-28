mod game;
mod circle;

use std::collections::VecDeque;

pub type Marble = usize;
pub type Score = usize;
pub type Id = usize;

#[derive(Debug)]
pub struct Player {
    pub id: Id,
    pub score: Score,
}

#[derive(Debug, Copy, Clone)]
pub struct GameRules {
    pub num_players: usize,
    pub last_marble_score: Score,
}

#[derive(Debug)]
pub struct Circle {
    marbles: VecDeque<Marble>,
    current_marble_index: usize,
    next_marble: Marble,
}

pub struct Game {
    rules: GameRules,
    players: Vec<Player>,
    current_player_id: Id,
    circle: Circle,
    round: usize,
    finished: bool,
}
