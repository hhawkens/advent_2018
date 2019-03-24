use super::types::*;
use chrono::prelude::*;
use regex::Regex;

pub fn get_guard_events() -> Vec<GuardEvent> {
    let mut guard_events = Vec::new();
    let guard_events_text = super::data::GUARD_SLEEP_TIMES;

    guard_events_text.lines().for_each(|l| {
        let time = get_time_from_line(l);
        let id = get_id_from_line(l);
        let action = get_action_from_line(l);
        guard_events.push(GuardEvent { id, time, action });
    });

    guard_events.sort();
    assign_ids_to_guard_events(&mut guard_events);

    guard_events
}

fn get_time_from_line(guard_event_text: &str) -> Time {
    let re = Regex::new(r"\[.*]").unwrap();
    let time_text = re.find(guard_event_text).unwrap().as_str().replace("[", "").replace("]", "");

    Utc.datetime_from_str(time_text.as_str(), "%Y-%m-%d %H:%M").unwrap()
}

fn get_id_from_line(guard_event_text: &str) -> i32 {
    let re = Regex::new(r"#[0-9]*").unwrap();
    let id_match = re.find(guard_event_text);

    match id_match {
        Some(s) => s.as_str().replace("#", "").parse::<i32>().unwrap(),
        None => -1,
    }
}

fn get_action_from_line(guard_event_text: &str) -> GuardAction {
    if guard_event_text.contains("begins shift") {
        GuardAction::BeginsShift
    } else if guard_event_text.contains("falls asleep") {
        GuardAction::FallsAsleep
    } else if guard_event_text.contains("wakes up") {
        GuardAction::WakesUp
    } else {
        panic!("No guard action found!");
    }
}

fn assign_ids_to_guard_events(sorted_guard_events: &mut Vec<GuardEvent>) {
    let mut last_id = 0;
    for event in sorted_guard_events {
        if event.id < 0 {
            event.id = last_id;
        } else {
            last_id = event.id;
        }
    }
}
