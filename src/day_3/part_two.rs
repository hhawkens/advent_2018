use super::types::*;

pub fn are_claims_overlapping(first: &Rect, second: &Rect) -> bool { // TODO private
    overlap_horizontally(first, second) && overlap_vertically(first, second)
}

fn overlap_horizontally(first: &Rect, second: &Rect) -> bool {
    let (leftmost, rightmost) = get_leftmost_and_rightmost_location(first, second);
    leftmost.location.x + leftmost.size.w >= rightmost.location.x
}

fn get_leftmost_and_rightmost_location<'a>(first: &'a Rect, second: &'a Rect) -> (&'a Rect, &'a Rect) {
    if first.location.x < second.location.x {
        (first, second)
    } else {
        (second, first)
    }
}

fn overlap_vertically(first: &Rect, second: &Rect) -> bool {
    let (highest, lowest) = get_highest_and_lowest_location(first, second);
    highest.location.y + highest.size.h >= lowest.location.y
}

fn get_highest_and_lowest_location<'a>(first: &'a Rect, second: &'a Rect) -> (&'a Rect, &'a Rect) {
    if first.location.y < second.location.y {
        (first, second)
    } else {
        (second, first)
    }
}
