use std::str::FromStr;

use anyhow::{Error, Result, anyhow};

pub fn run(input: &str) -> Result<usize> {
    let mut count: usize = 0;

    let ranges = input
        .trim()
        .split(",")
        .into_iter()
        .map(|range| {
            let mut split = range.split("-");
            let a = split.next().ok_or(anyhow!("No first value! {range}"))?;
            let b = split.next().ok_or(anyhow!("No second value! {range}"))?;

            if a.starts_with("0") || b.starts_with("0") {
                return Err(anyhow!("a or b starts with 0! {range}"));
            }

            if split.next().is_some() {
                return Err(anyhow!("Third value in range! {range}"));
            }
            let a = FromStr::from_str(a).unwrap();
            let b = FromStr::from_str(b).unwrap();

            Ok((a, b))
        })
        .collect::<Result<Vec<(usize, usize)>, Error>>()?;

    for (first, last) in ranges {
        for i in first..=last {
            if is_invalid(&i.to_string()) {
                count += i;
            }
        }
    }

    Ok(count)
}

fn is_invalid(val: &String) -> bool {
    let mut part = String::default();

    for char in val.chars() {
        part.push(char);

        if part == *val {
            return false;
        }

        let split = val.split(&part).filter(|s| !s.is_empty()).count();

        if split == 0 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn run_test() {
        let input = include_str!("../test_input.txt");

        let output = run(input).unwrap();

        assert_eq!(output, 4174379265);
    }
}
