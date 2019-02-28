use super::types::*;

/// Parses the game rules text to internal data representation
pub fn get_game_rules(rules_text: &str) -> GameRules {
    let regex = regex::Regex::new(r"([0-9])\w+").unwrap();
    let mut matches = regex.find_iter(rules_text);
    let num_players = matches.next().unwrap().as_str().parse().unwrap();
    let last_marble_points = matches.next().unwrap().as_str().parse().unwrap();
    GameRules {
        num_players,
        num_marbles: last_marble_points
    }
}

/// If an index is out of bounds for given collection, this function returns a valid index, as if the
/// collection was circular
pub fn circular_index(index: isize, sequence: impl IntoIterator) -> usize {
    let size = sequence.into_iter().count() as isize;
    let rem = (index % size).abs();

    if index >= 0 || rem == 0 {
        rem as usize
    } else {
        (size - rem) as usize
    }
}
