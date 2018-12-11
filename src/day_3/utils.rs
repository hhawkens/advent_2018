use std::collections::HashMap;
use super::types::*;

pub fn get_fabric_claims() -> HashMap<i32, Rect> {
    let mut id_rect_map = HashMap::new();
    let fabric_claim_str = super::data::FABRIC_CLAIMS;
    fabric_claim_str
        .lines()
        .for_each(|l| {
            id_rect_map.insert(
                get_id(l),
                Rect { size: get_rect_size(l), location: get_rect_location(l) });
        });

    id_rect_map
}

fn get_id(_claim: &str) -> i32 {
    0
}

fn get_rect_location(_claim: &str) -> Point {
    Point { x: 0, y: 0 }
}

fn get_rect_size(_claim: &str) -> Size {
    Size { w: 0, h: 0 }
}
