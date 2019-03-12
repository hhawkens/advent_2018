use super::types::*;
use regex::Regex;
use std::collections::HashMap;

pub fn get_fabric_claims() -> HashMap<i32, Rect> {
    let mut id_rect_map = HashMap::new();
    let fabric_claim_str = super::data::FABRIC_CLAIMS;
    fabric_claim_str.lines().for_each(|l| {
        id_rect_map.insert(
            get_id(l),
            Rect {
                size: get_rect_size(l),
                location: get_rect_location(l),
            },
        );
    });

    id_rect_map
}

fn get_id(claim: &str) -> i32 {
    let re = Regex::new(r"#[0-9]*").unwrap();
    re.find(claim).unwrap().as_str().replace("#", "").parse().unwrap()
}

fn get_rect_location(claim: &str) -> Point {
    let re = Regex::new(r"[0-9]*,[0-9]*").unwrap();
    let location: Vec<&str> = re.find(claim).unwrap().as_str().split(',').collect();
    Point {
        x: location[0].parse().unwrap(),
        y: location[1].parse().unwrap(),
    }
}

fn get_rect_size(claim: &str) -> Size {
    let re = Regex::new(r"[0-9]*x[0-9]*").unwrap();
    let size: Vec<&str> = re.find(claim).unwrap().as_str().split('x').collect();
    Size {
        w: size[0].parse().unwrap(),
        h: size[1].parse().unwrap(),
    }
}
