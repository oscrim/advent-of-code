use std::time::Instant;

use d9_lib::*;

fn main() {
    let input = INPUT;
    let now = Instant::now();
    let a_output = a::run(input);
    let b_output = b::run(input);
    let elapsed = now.elapsed().as_millis();
    println!("Part 1: {a_output}");
    println!("Part 2: {b_output}");
    println!("Finished in {elapsed}ms")
}
