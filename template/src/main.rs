use d_lib::*;

fn main() {
    let input = INPUT;
    let a_output = a::run(input);
    let b_output = b::run(input);
    println!("Part 1: {a_output}");
    println!("Part 2: {b_output}");
}
