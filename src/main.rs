mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

fn main() {
    let mut sw = stopwatch::Stopwatch::start_new();

    let result_1_1 = day_1::result_part_one();
    println!("Result_1_1: {}", result_1_1);

    let result_1_2 = day_1::result_part_two();
    println!("Result_1_2: {}", result_1_2);

    let result_2_1 = day_2::result_part_one();
    println!("Result_2_1: {}", result_2_1);

    let result_2_2 = day_2::result_part_two();
    println!("Result_2_2: {}", result_2_2);

    let result_3_1 = day_3::result_part_one();
    println!("Result_3_1: {}", result_3_1);

    let result_3_2 = day_3::result_part_two();
    println!("Result_3_2: {}", result_3_2);

    let result_4_1 = day_4::result_part_one();
    println!("Result_4_1: {}", result_4_1);

    let result_4_2 = day_4::result_part_two();
    println!("Result_4_2: {}", result_4_2);

    let result_5_1 = day_5::result_part_one();
    println!("Result_5_1: {}", result_5_1);

    let result_5_2 = day_5::result_part_two();
    println!("Result_5_2: {}", result_5_2);

    sw.stop();
    println!("\nDuration: {:?}", sw.elapsed())
}
