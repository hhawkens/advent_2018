use super::types::*;
use regex::Regex;

pub fn get_game_rules(rules_text: &str) -> GameRules {
    let regex = Regex::new(r"([0-9])\w+").unwrap();
    let mut matches = regex.find_iter(rules_text);
    let num_players = matches.next().unwrap().as_str().parse().unwrap();
    let last_marble_points = matches.next().unwrap().as_str().parse().unwrap();
    GameRules {
        num_players,
        last_marble_points
    }
}
