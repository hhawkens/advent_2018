use rayon::prelude::*;
use std::cmp::min;

pub fn react_polymer_parallel(polymer: &str) -> String {
    if polymer.len() < 2500 {
        return react_polymer(polymer);
    }

    let mut reacted_polymer = polymer.to_string();
    let mut skip_loop_count = 0;

    for thread_count in (2..25).rev() {
        if skip_loop_count > 0 {
            skip_loop_count -= 1;
            continue;
        }

        let text_chunks = split_string_into_chunks(&reacted_polymer, thread_count);
        let polymer_count_before = reacted_polymer.len();

        reacted_polymer = text_chunks
            .par_iter()
            .map(|text_chunk| react_polymer(text_chunk))
            .collect();

        let polymer_count_after = reacted_polymer.len();

        if polymer_count_before == polymer_count_after {
            skip_loop_count = 3;
        }
    }

    react_polymer(&reacted_polymer)
}

fn react_polymer(polymer: &str) -> String {
    let mut reacted_polymer = polymer.to_string();

    loop {
        let last_index = reacted_polymer.chars().count();
        let mut skip_one_turn = false;
        for char_index in (1..last_index).rev() {
            if skip_one_turn {
                skip_one_turn = false;
                continue;
            }

            let first = reacted_polymer.chars().nth(char_index).unwrap();
            let second = reacted_polymer.chars().nth(char_index - 1).unwrap();
            if do_polymer_parts_react(first, second) {
                reacted_polymer.remove(char_index);
                reacted_polymer.remove(char_index - 1);
                skip_one_turn = true;
            }
        }

        if reacted_polymer.chars().count() == last_index {
            break;
        }
    }

    reacted_polymer
}

fn do_polymer_parts_react(first: char, second: char) -> bool {
    let are_same = first == second;
    let are_equal = first.to_lowercase().to_string() == second.to_lowercase().to_string();

    are_equal && !are_same
}

fn split_string_into_chunks(text: &str, chunk_count: usize) -> Vec<&str> {
    let chars_per_chunk = (text.len() as f64 / chunk_count as f64).ceil() as usize;
    let mut chunks = vec![];
    let mut cur = text;
    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(min(chars_per_chunk, cur.len()));
        chunks.push(chunk);
        cur = rest;
    }

    chunks
}
