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
    let mut first = 0_usize;
    let mut second = 0_usize;

    let chars_len = bank.chars().count();

    for (i, jolt) in bank.chars().enumerate() {
        let jolt = jolt.to_digit(10).unwrap() as usize;

        if i < chars_len - 1 && jolt > first {
            first = jolt;
            second = 0;
            continue;
        } else if jolt > second {
            second = jolt;
        }
    }

    let mut output = first.to_string();
    output.push_str(&second.to_string());

    FromStr::from_str(&output).unwrap()
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn test_run() {
        let input = include_str!("../test_input.txt");

        let output = run(input);

        assert_eq!(output, 357);
    }
}
