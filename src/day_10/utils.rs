use crate::day_10::types::*;
use std::iter::FromIterator;

pub fn get_stars<I: FromIterator<Star>>(stars_text: &str) -> I {
    let regex = regex::Regex::new(r"-*[0-9]+").unwrap();
    let mut matches = regex.find_iter(stars_text);

    stars_text
        .lines()
        .map(|_| {
            let l_x = matches.next().unwrap().as_str().parse().unwrap();
            let l_y = matches.next().unwrap().as_str().parse().unwrap();
            let v_x = matches.next().unwrap().as_str().parse().unwrap();
            let v_y = matches.next().unwrap().as_str().parse().unwrap();
            let location = Point { x: l_x, y: l_y };
            let velocity = Velocity { x: v_x, y: v_y };
            Star { location, velocity }
        })
        .collect()
}
