mod day_1;
mod day_2;
mod day_3;
use self::day_3::types::*;

fn main() {
//    let result_1_1 = day_1::result_part_one();
//    println!("Result_1_1: {}", result_1_1);
//
//    let result_1_2 = day_1::result_part_two();
//    println!("Result_1_2: {}", result_1_2);
//
//    let result_2_1 = day_2::result_part_one();
//    println!("Result_2_1: {}", result_2_1);
//
//    let result_2_2 = day_2::result_part_two();
//    println!("Result_2_2: {}", result_2_2);
//
//    let result_3_1 = day_3::result_part_one();
//    println!("Result_3_1: {:?}", result_3_1);


    let r1: Rect = Rect { location: Point {x:3, y:2}, size: Size {w: 5, h: 4} };
    let r2: Rect = Rect { location: Point {x:1, y:1}, size: Size {w: 6, h: 1} };
    let result_1 = day_3::part_two::are_claims_overlapping(&r1, &r2);
    let result_2 = day_3::part_two::are_claims_overlapping(&r2, &r1);
    println!("Result_3_1: {} {}", result_1, result_2);
}
