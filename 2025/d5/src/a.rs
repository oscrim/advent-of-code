use std::str::FromStr;

pub fn run(input: &str) -> usize {
    let mut lines = input.lines();
    let mut fresh_ranges = Vec::default();
    let mut fresh_ingredients = 0;

    loop {
        let line = lines.next().unwrap();

        if line == "" {
            break;
        }

        let mut split = line.split('-');
        let first = split.next().unwrap();
        let last = split.next().unwrap();

        let first: usize = FromStr::from_str(first).unwrap();
        let last: usize = FromStr::from_str(last).unwrap();

        let range = first..=last;
        fresh_ranges.push(range);
    }

    for line in lines {
        let id: usize = FromStr::from_str(line).unwrap();

        for fresh_range in &fresh_ranges {
            if fresh_range.contains(&id) {
                #[cfg(test)]
                println!("Fresh: {id}");
                fresh_ingredients += 1;
                break;
            }
        }
    }

    fresh_ingredients
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test() {
        let input = include_str!("../test-input.txt");

        let output = run(input);

        assert_eq!(output, 3);
    }
}
