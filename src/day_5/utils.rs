use rayon::prelude::*;
use std::cmp::min;

const TASK_COUNT: usize = 26;

pub fn chain_react_polymer(polymer: &str) -> String {
    if polymer.len() < 5000 {
        return react_polymer(polymer, &react_polymer_one_iteration);
    }

    let mut reacted_polymer = polymer.to_string();

    for task_count in (2..TASK_COUNT).rev() {
        let text_chunks = split_string_into_chunks(&reacted_polymer, task_count);

        reacted_polymer = text_chunks
            .par_iter()
            .map(|text_chunk| react_polymer(text_chunk, &react_polymer_one_iteration_parallel))
            .collect();
    }

    react_polymer(&reacted_polymer, &react_polymer_one_iteration)
}

fn react_polymer(polymer: &str, reaction_fn: &Fn(&str) -> String) -> String {
    let mut reacted_polymer = polymer.to_string();
    let mut size_before = 0;
    let mut size_after = 1;

    while size_before != size_after {
        size_before = reacted_polymer.len();
        reacted_polymer = reaction_fn(&reacted_polymer);
        size_after = reacted_polymer.len();
    }

    reacted_polymer
}

fn react_polymer_one_iteration_parallel(polymer: &str) -> String {
    let mut reacted_polymer = polymer.to_string();
    let text_chunks = split_string_into_chunks(&reacted_polymer, TASK_COUNT / 3);

    reacted_polymer = text_chunks
        .par_iter()
        .map(|text_chunk| react_polymer_one_iteration(text_chunk))
        .collect();

    reacted_polymer
}

fn react_polymer_one_iteration(polymer: &str) -> String {
    let mut reacted_polymer = polymer.to_string();

    let last_index = reacted_polymer.chars().count();
    let mut skip_one_turn = false;
    for char_index in (1..last_index).rev() {
        if skip_one_turn {
            skip_one_turn = false;
            continue;
        }

        let first = reacted_polymer.chars().nth(char_index).unwrap();
        let second = reacted_polymer.chars().nth(char_index - 1).unwrap();
        if do_polymer_units_react(first, second) {
            reacted_polymer.remove(char_index);
            reacted_polymer.remove(char_index - 1);
            skip_one_turn = true;
        }
    }

    reacted_polymer
}

fn do_polymer_units_react(first: char, second: char) -> bool {
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
