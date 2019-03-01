use std::collections::VecDeque;

/// Parses the game rules text to internal data representation
pub fn get_game_rules(rules_text: &str) -> (usize, usize) {
    let regex = regex::Regex::new(r"([0-9])\w+").unwrap();
    let mut matches = regex.find_iter(rules_text);
    let num_players = matches.next().unwrap().as_str().parse().unwrap();
    let num_marbles = matches.next().unwrap().as_str().parse().unwrap();
    (num_players, num_marbles)
}

pub fn play_marble_game(num_players: usize, num_marbles: usize) -> usize {
    let mut circle = VecDeque::with_capacity(num_marbles);
    circle.push_front(0);
    let mut scores = vec![0_usize; num_players];

    for marble in 1..num_marbles {
        if marble % 23 == 0 {
            rotate(&mut circle, 7);
            let score_m = marble;
            let score_p = circle.pop_front().unwrap();
            let score = score_m + score_p;
            scores[marble % num_players] += score;
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
    if deq.len() == 0 {
        return;
    }

    let abs = n.abs() as usize % deq.len();
    if n >= 0 {
        for _ in 0..abs {
            let popped = deq.pop_back();
            match popped {
                Some(val) => deq.push_front(val),
                None => {},
            }
        }
    } else {
        for _ in 0..abs {
            let popped = deq.pop_front();
            match popped {
                Some(val) => deq.push_back(val),
                None => {},
            }
        }
    }
}
