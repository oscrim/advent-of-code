use std::str::FromStr;

pub fn run(input: &str) -> usize {
    let banks = input.lines();

    let mut joltage = 0;

    for bank in banks {
        joltage += calculate_bank_joltage(bank);
    }

    joltage
}

fn calculate_bank_joltage<'a>(bank: &str) -> usize {
    let mut output = String::default();
    let mut index: usize = 0;

    let chars_len = bank.chars().count();

    for digit in 0..12 {
        let cutoff_index = chars_len - 11 + digit;

        #[cfg(test)]
        {
            println!("Digit: {digit}");
            println!("Index: {index}");
            println!("Cutoff: {cutoff_index}");
        }

        let slice = &bank[index..cutoff_index];

        #[cfg(test)]
        {
            let remainder = &bank[cutoff_index..];
            let previous = &bank[0..index];

            println!("Previous: {previous}");
            println!("Slice: {slice}");
            println!("Remainder: {remainder}");
        }

        let mut max: usize = 0;
        let mut max_index: usize = 0;

        for (i, jolt) in slice.chars().enumerate() {
            let jolt = jolt.to_digit(10).unwrap() as usize;

            if jolt > max {
                max = jolt;
                max_index = index + i;
            }
        }

        index = max_index + 1;
        output.push_str(&max.to_string());
    }

    FromStr::from_str(&output).unwrap()
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn test_run() {
        let input = include_str!("../test_input.txt");

        let output = run(input);

        assert_eq!(output, 3121910778619);
    }
}
