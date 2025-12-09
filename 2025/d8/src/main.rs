use d8_lib::*;

fn main() {
    let input = INPUT;
    let a_output = a::run(input, 1000);
    let b_output = b::run(input);
    println!("{a_output}");
    println!("{b_output}");
}
