mod day_1;
mod day_2;
mod day_3;

fn main() {
    let result_1_1 = day_1::result_part_one();
    println!("Result_1_1: {}", result_1_1);

    let result_1_2 = day_1::result_part_two();
    println!("Result_1_2: {}", result_1_2);

    let result_2_1 = day_2::result_part_one();
    println!("Result_2_1: {}", result_2_1);

    let result_2_2 = day_2::result_part_two();
    println!("Result_2_2: {}", result_2_2);

    let test_3_1 = day_3::utils::get_fabric_claims();
    println!("Test_3_1: {:?}", test_3_1);
}
