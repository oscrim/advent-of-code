use std::str::FromStr;

pub fn run(input: &str) -> usize {
    let mut lines = input.lines();
    let mut numbers: Vec<Vec<usize>> = Vec::default();
    let mut operations: Vec<&str> = Vec::default();

    let first_line = lines.next().unwrap();

    for number in first_line.split(" ").filter(|v| !v.is_empty()) {
        let number: usize = FromStr::from_str(number).unwrap();

        numbers.push(vec![number]);
    }

    for line in lines {
        let split = line
            .split(" ")
            .into_iter()
            .filter(|v| !v.is_empty())
            .enumerate();

        for (i, val) in split {
            match val {
                v if v == "*" || v == "+" => {
                    operations.push(v);
                }
                d => {
                    numbers[i].push(FromStr::from_str(d).unwrap());
                }
            }
        }
    }

    let mut sum = 0;

    for (i, numbers) in numbers.into_iter().enumerate() {
        let operation = operations[i];

        match operation {
            "*" => {
                let mut total = 1;

                for number in numbers {
                    total *= number;
                }

                sum += total;
            }
            "+" => {
                let mut total = 0;

                for number in numbers {
                    total += number;
                }

                sum += total;
            }
            _ => unreachable!(),
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test() {
        let input = include_str!("../test-input.txt");

        let output = run(input);

        assert_eq!(output, 4277556);
    }
}
