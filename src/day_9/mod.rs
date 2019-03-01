use crate::day_9::utils::play_marble_game;

mod data;
mod utils;

pub fn result_part_one() -> usize {
    let (num_players, num_marbles) = utils::get_game_rules(data::GAME_RULES);
    let high_score = play_marble_game(num_players, num_marbles);
    high_score
}
