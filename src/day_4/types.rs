use chrono::prelude::*;
use std::cmp::Ordering;
pub type Time = DateTime<Utc>;

#[derive(Debug, Eq, PartialEq)]
pub enum GuardAction {
    BeginsShift,
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, Eq, PartialEq)]
pub struct GuardEvent {
    pub id: i32,
    pub time: Time,
    pub action: GuardAction,
}

impl Ord for GuardEvent {
    fn cmp(&self, other: &GuardEvent) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for GuardEvent {
    fn partial_cmp(&self, other: &GuardEvent) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}
