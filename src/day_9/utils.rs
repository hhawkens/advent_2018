use std::collections::VecDeque;

/// Parses the game rules text to internal data representation
pub fn get_game_rules(rules_text: &str) -> (u64, u64) {
    let regex = regex::Regex::new(r"([0-9])\w+").unwrap();
    let mut matches = regex.find_iter(rules_text);
    let num_players = matches.next().unwrap().as_str().parse().unwrap();
    let num_marbles = matches.next().unwrap().as_str().parse().unwrap();
    (num_players, num_marbles)
}

pub fn play_marble_game(num_players: u64, num_marbles: u64) -> u64 {
    let mut circle = VecDeque::with_capacity(num_marbles as usize);
    circle.push_front(0);
    let mut scores = vec![0_u64; num_players as usize];

    for marble in 1..num_marbles {
        if marble % 23 == 0 {
            rotate(&mut circle, 7);
            let score_m = marble;
            let score_p = circle.pop_front().unwrap();
            let score = score_m + score_p;
            scores[(marble % num_players) as usize] += score;
        } else {
            rotate(&mut circle, -2);
            circle.push_front(marble);
        }
    }

    *scores.iter().max().unwrap()
}

/// Rotates given deque to the right(positive n) or to the left(negative n), emulating a circular collection
/// (This function was not available in stable std at the time of writing)
fn rotate<T>(deq: &mut VecDeque<T>, n: isize) {
    if deq.is_empty() {
        return;
    }

    let abs = n.abs() as u64 % deq.len() as u64;
    if n >= 0 {
        for _ in 0..abs {
            if let Some(val) = deq.pop_back() {
                deq.push_front(val)
            }
        }
    } else {
        for _ in 0..abs {
            if let Some(val) = deq.pop_front() {
                deq.push_back(val)
            }
        }
    }
}
