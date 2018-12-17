use super::types::*;

pub fn find_claim_id_without_overlap() -> i32 {
    let claims = super::utils::get_fabric_claims();
    for (id_outer, claim_outer) in &claims {
        let mut found_overlap = false;
        for (id_inner, claim_inner) in &claims {
            if *id_inner == *id_outer {
                continue
            }
            if are_claims_overlapping(claim_outer, claim_inner) {
                found_overlap = true;
                break
            }
        }

        if found_overlap {
            continue
        } else {
            return *id_outer;
        }
    }
    -1
}

fn are_claims_overlapping(first: &Rect, second: &Rect) -> bool {
    are_overlapping_horizontally(first, second) && are_overlapping_vertically(first, second)
}

fn are_overlapping_horizontally(first: &Rect, second: &Rect) -> bool {
    let (leftmost, rightmost) = get_leftmost_and_rightmost_location(first, second);
    leftmost.location.x + leftmost.size.w > rightmost.location.x
}

fn get_leftmost_and_rightmost_location<'a>(first: &'a Rect, second: &'a Rect) -> (&'a Rect, &'a Rect) {
    if first.location.x < second.location.x {
        (first, second)
    } else {
        (second, first)
    }
}

fn are_overlapping_vertically(first: &Rect, second: &Rect) -> bool {
    let (highest, lowest) = get_highest_and_lowest_location(first, second);
    highest.location.y + highest.size.h > lowest.location.y
}

fn get_highest_and_lowest_location<'a>(first: &'a Rect, second: &'a Rect) -> (&'a Rect, &'a Rect) {
    if first.location.y < second.location.y {
        (first, second)
    } else {
        (second, first)
    }
}
