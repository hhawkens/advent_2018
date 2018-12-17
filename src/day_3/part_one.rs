use super::types::*;
use std::collections::HashMap;

pub fn multiple_claims_count() -> i32 {
    let claims = super::utils::get_fabric_claims();
    let claim_count_map = get_claim_count_for_points(claims.values());

    claim_count_map.values().filter(|&&c| c > 1).count() as i32
}

fn get_claim_count_for_points<'a, I: IntoIterator<Item = &'a Rect>>(
    claims: I,
) -> HashMap<Point, i32> {
    let mut claim_count_map = HashMap::new();

    for claim in claims.into_iter() {
        let claimed_inches = get_claimed_inches(claim);
        for claimed_inch in claimed_inches {
            *claim_count_map.entry(claimed_inch).or_insert(0) += 1;
        }
    }
    claim_count_map
}

fn get_claimed_inches(claim: &Rect) -> Vec<Point> {
    let mut result = Vec::new();
    let loc = &claim.location;
    let size = &claim.size;

    for vertical in loc.y..loc.y + size.h {
        for horizontal in loc.x..loc.x + size.w {
            result.push(Point {
                x: horizontal,
                y: vertical,
            })
        }
    }
    result
}
