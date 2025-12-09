use std::time::{Duration, Instant};

use d1_lib;
use d2_lib;
use d3_lib;
use d4_lib;
use d5_lib;
use d6_lib;
use d7_lib;
use d8_lib;
use d9_lib;

fn main() {
    let input_1 = d1_lib::INPUT;
    let input_2 = d2_lib::INPUT;
    let input_3 = d3_lib::INPUT;
    let input_4 = d4_lib::INPUT;
    let input_5 = d5_lib::INPUT;
    let input_6 = d6_lib::INPUT;
    let input_7 = d7_lib::INPUT;
    let input_8 = d8_lib::INPUT;
    let input_9 = d9_lib::INPUT;

    let mut times = Vec::with_capacity(12);

    println!("Day 1");
    let start = Instant::now();

    d1_lib::a::run(input_1);
    d1_lib::b::run(input_1);

    let day1_elapsed = start.elapsed();
    println!("Elapsed: {}μs", day1_elapsed.as_micros());
    times.push(day1_elapsed);

    println!("Day 2");
    let start = Instant::now();

    d2_lib::a::run(input_2).unwrap();
    d2_lib::b::run(input_2).unwrap();

    let day2_elapsed = start.elapsed();
    println!("Elapsed: {}ms", day2_elapsed.as_millis());
    times.push(day2_elapsed);

    println!("Day 3");
    let start = Instant::now();

    d3_lib::a::run(input_3);
    d3_lib::b::run(input_3);

    let day3_elapsed = start.elapsed();
    println!("Elapsed: {}μs", day3_elapsed.as_micros());
    times.push(day3_elapsed);

    println!("Day 4");
    let start = Instant::now();

    d4_lib::a::run(input_4);
    d4_lib::b::run(input_4);

    let day4_elapsed = start.elapsed();
    println!("Elapsed: {}μs", day4_elapsed.as_micros());
    times.push(day4_elapsed);

    println!("Day 5");
    let start = Instant::now();

    d5_lib::a::run(input_5);
    d5_lib::b::run(input_5);

    let day5_elapsed = start.elapsed();
    println!("Elapsed: {}μs", day5_elapsed.as_micros());
    times.push(day5_elapsed);

    println!("Day 6");
    let start = Instant::now();

    d6_lib::a::run(input_6);
    d6_lib::b::run(input_6);

    let day6_elapsed = start.elapsed();
    println!("Elapsed: {}μs", day6_elapsed.as_micros());
    times.push(day6_elapsed);

    println!("Day 7");
    let start = Instant::now();

    d7_lib::a::run(input_7);
    d7_lib::b::run(input_7);

    let day7_elapsed = start.elapsed();
    println!("Elapsed: {}μs", day7_elapsed.as_micros());
    times.push(day7_elapsed);

    println!("Day 8");
    let start = Instant::now();

    d8_lib::a::run(input_8, 1000);
    d8_lib::b::run(input_8);

    let day8_elapsed = start.elapsed();
    println!("Elapsed: {}ms", day8_elapsed.as_millis());
    times.push(day8_elapsed);

    println!("Day 9");
    let start = Instant::now();

    d9_lib::a::run(input_9);
    d9_lib::b::run(input_9);

    let day9_elapsed = start.elapsed();
    println!("Elapsed: {:.3}s", day9_elapsed.as_secs_f32());
    times.push(day9_elapsed);

    let total: Duration = times.iter().sum();

    println!("Total Elapsed: {:.3}s", total.as_secs_f32());
}
