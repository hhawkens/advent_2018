use super::types::{GuardAction, GuardEvent};
use chrono::prelude::*;
use std::collections::HashMap;

pub fn get_most_slept_minute_for_guard(id: i32, guard_events: &[GuardEvent]) -> u32 {
    let mut minutes_slept_map = HashMap::new();
    let mut start_minute = 0;

    guard_events
        .iter()
        .filter(|&ev| ev.id == id)
        .for_each(|ev| match ev.action {
            GuardAction::FallsAsleep => start_minute = ev.time.minute(),
            GuardAction::WakesUp => {
                for m in start_minute..ev.time.minute() {
                    *minutes_slept_map.entry(m).or_insert(0) += 1;
                }
            }
            _ => {}
        });

    get_most_slept_minute(&minutes_slept_map)
}

pub fn get_guard_with_most_sleep_time(guard_events: &[GuardEvent]) -> i32 {
    let mut guard_id_sleep_minutes_map = HashMap::new();
    let mut last_falling_asleep_minute = 0u32;

    for guard_event in guard_events {
        match guard_event.action {
            GuardAction::FallsAsleep => last_falling_asleep_minute = guard_event.time.minute(),
            GuardAction::WakesUp => {
                let guard_id = guard_event.id;
                let sleep_minutes = guard_event.time.minute() - last_falling_asleep_minute;
                *guard_id_sleep_minutes_map.entry(guard_id).or_insert(0) += sleep_minutes;
            }
            _ => {}
        }
    }

    get_key_of_highest_value(&guard_id_sleep_minutes_map)
}

fn get_key_of_highest_value(map: &HashMap<i32, u32>) -> i32 {
    let mut max_id = -1;
    let mut max_val = 0u32;

    for (id, value) in map {
        if *value > max_val {
            max_val = *value;
            max_id = *id;
        }
    }

    max_id
}

fn get_most_slept_minute(map: &HashMap<u32, i32>) -> u32 {
    let mut max_minute = 0;
    let mut max_val = 0;

    for (minute, count) in map {
        if *count > max_val {
            max_val = *count;
            max_minute = *minute;
        }
    }

    max_minute
}
