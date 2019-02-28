mod types;
mod data;
mod utils;

use self::types::*;

pub fn result_part_one() -> Score {
    let rules = GameRules{num_players: 10, last_marble_score: 1618};
    let mut game = Game::new(rules);
    let winner = game.play();
    dbg!(&winner);
    0
}
