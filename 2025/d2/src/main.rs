use d2_lib::*;

fn main() -> anyhow::Result<()> {
    let input = INPUT;
    let a_output = a::run(input)?;
    let b_output = b::run(input)?;
    println!("{a_output}");
    println!("{b_output}");

    Ok(())
}
