use super::types::GuardAction;
use super::utils::*;
use chrono::prelude::*;
use std::collections::HashMap;

pub fn get_guard_with_most_sleep_time() -> i32 {
    let guard_events = get_guard_events();
    let mut guard_id_sleep_minutes_map = HashMap::<i32, u32>::new();
    let mut last_falling_asleep_minute = 0u32;

    for guard_event in &guard_events {
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
