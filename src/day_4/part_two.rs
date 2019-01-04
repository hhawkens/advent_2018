use chrono::prelude::*;
use std::collections::HashMap;
use super::types::*;

pub fn get_guard_and_most_slept_minute() -> (i32, i32) {
    let guard_sleep_minutes = get_guard_sleep_minutes();
    let mut highest_count = -1;
    let mut guard_result = -1;
    let mut minute_result = -1;

    for (guard_id, sleep_minutes) in guard_sleep_minutes {
        for (sleep_minute, count) in sleep_minutes {
            if count > highest_count {
                guard_result = guard_id;
                minute_result = sleep_minute;
            }
        }
    }

    (guard_result, minute_result)
}

pub fn get_guard_sleep_minutes() -> HashMap<i32, GuardSleepMinutes> {
    let guard_events = super::utils::get_guard_events();
    let mut start_sleep_event: Option<&GuardEvent> = None;
    let mut guard_sleep_minutes = HashMap::new();

    for guard_event in &guard_events {
        match guard_event.action {
            GuardAction::FallsAsleep => start_sleep_event = Some(guard_event),
            GuardAction::WakesUp => {
                let current_sleep_minutes = guard_sleep_minutes
                    .entry(guard_event.id)
                    .or_insert(GuardSleepMinutes::new());

                let start = start_sleep_event.unwrap().time.minute();
                let end = guard_event.time.minute();

                for minute in start..end {
                    *current_sleep_minutes.entry(minute as i32).or_insert(0) += 1;
                }
            },
            _ => { }
        }
    }

    guard_sleep_minutes
}
