use std::str::FromStr;

pub fn run(input: &str) -> usize {
    let mut operations: Vec<_> = input
        .lines()
        .last()
        .unwrap()
        .split(" ")
        .filter(|v| !v.is_empty())
        .collect();
    operations.reverse();
    let mut problems: Vec<Vec<String>> = Vec::default();

    let mut a: Vec<_> = input
        .lines()
        .filter_map(|line| {
            if line.contains('+') || line.contains('*') {
                None
            } else {
                Some(line.chars().rev())
            }
        })
        .collect();

    'outer: loop {
        let mut problem = Vec::new();

        loop {
            let mut number = String::default();
            for line in a.iter_mut() {
                if let Some(c) = line.next() {
                    match c {
                        ' ' if number.is_empty() => {
                            number.push('0');
                        }
                        ' ' if !number.is_empty() => {}
                        d => {
                            number.push(d);
                        }
                    }
                } else {
                    problems.push(problem);
                    break 'outer;
                }
            }

            let actual_number: usize = FromStr::from_str(&number).unwrap();

            if actual_number == 0 {
                break;
            } else {
                problem.push(number);
            }
        }
        problems.push(problem);
    }

    #[cfg(test)]
    {
        println!("Problems: {problems:?}");
        println!("Operations: {operations:?}");
    }

    let numbers: Vec<Vec<usize>> = problems
        .into_iter()
        .map(|vals| {
            vals.into_iter()
                .map(|val| FromStr::from_str(&val).unwrap())
                .collect()
        })
        .collect();

    calculate_sum(numbers, operations)
}

fn calculate_sum(numbers: Vec<Vec<usize>>, operations: Vec<&str>) -> usize {
    let mut sum = 0;

    for (i, numbers) in numbers.into_iter().enumerate() {
        let operation = operations[i];

        #[cfg(test)]
        println!("Applying {operation} to {numbers:?}");

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

        assert_eq!(output, 3263827);
    }
}
