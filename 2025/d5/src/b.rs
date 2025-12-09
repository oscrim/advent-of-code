use std::{ops::RangeInclusive, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, PartialOrd)]
struct Range {
    min: usize,
    max: usize,
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.min.cmp(&other.min);

        if let std::cmp::Ordering::Equal = cmp {
            self.max.cmp(&other.max)
        } else {
            cmp
        }
    }
}

impl Range {
    fn to_range(&self) -> RangeInclusive<usize> {
        self.min..=self.max
    }
    fn range_size(&self) -> usize {
        self.max - self.min + 1
    }
    fn combine(&mut self, other: Range) -> Option<Range> {
        let range = self.to_range();

        let contains_other_min = range.contains(&other.min);
        let contains_other_max = range.contains(&other.max);

        match (contains_other_min, contains_other_max) {
            (false, false) => {
                return Some(other);
            }
            (true, true) => {}
            (true, false) => {
                self.max = other.max;
            }
            (false, true) => {
                self.min = other.min;
            }
        }

        None
    }
}

pub fn run(input: &str) -> usize {
    let lines = input.lines();
    let mut fresh_ranges: Vec<Range> = Vec::new();

    for line in lines {
        if line == "" {
            break;
        }

        let mut split = line.split('-');
        let first = split.next().unwrap();
        let last = split.next().unwrap();

        let min: usize = FromStr::from_str(first).unwrap();
        let max: usize = FromStr::from_str(last).unwrap();

        fresh_ranges.push(Range { min, max });
    }

    fresh_ranges.sort_unstable();

    let mut new_ranges: Vec<Range> = Vec::new();

    while let Some(mut range) = fresh_ranges.first().copied() {
        if range == *fresh_ranges.last().unwrap() {
            new_ranges.push(range);
            break;
        }

        let mut remaining_ranges: Vec<Range> = Vec::new();

        for &other in fresh_ranges[1..].iter() {
            if let Some(remaining) = range.combine(other) {
                remaining_ranges.push(remaining);
            }
        }

        new_ranges.push(range);
        fresh_ranges = remaining_ranges;
    }

    #[cfg(test)]
    for range in &new_ranges {
        println!("{}-{}", range.min, range.max);
    }

    new_ranges
        .into_iter()
        .fold(0, |acc, range| acc + range.range_size())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test() {
        let input = include_str!("../test-input.txt");

        let output = run(input);

        assert_eq!(output, 14);
    }

    #[test]
    fn t1() {
        let mut r1 = Range { min: 0, max: 3 };

        let r2 = Range { min: 0, max: 3 };

        let out = r1.combine(r2);

        assert!(out.is_none());
    }

    #[test]
    fn t2() {
        let mut r1 = Range { min: 0, max: 3 };

        let r2 = Range { min: 1, max: 4 };

        let out = r1.combine(r2);

        assert!(out.is_none());
        assert_eq!(r1, Range { min: 0, max: 4 });
    }

    #[test]
    fn t3() {
        let mut r1 = Range { min: 2, max: 3 };

        let r2 = Range { min: 0, max: 3 };

        let out = r1.combine(r2);

        assert!(out.is_none());
        assert_eq!(r1, Range { min: 0, max: 3 });
    }

    #[test]
    fn t4() {
        let mut r1 = Range { min: 0, max: 3 };

        let r2 = Range { min: 5, max: 6 };

        let out = r1.combine(r2);

        assert!(out.is_some());
    }

    #[test]
    fn t5() {
        let mut r1 = Range { min: 0, max: 3 };

        let r2 = Range { min: 3, max: 6 };

        let out = r1.combine(r2);

        assert!(out.is_none());
        assert_eq!(r1, Range { min: 0, max: 6 });
    }
    #[test]
    fn t6() {
        let mut r1 = Range { min: 3, max: 6 };

        let r2 = Range { min: 0, max: 3 };

        let out = r1.combine(r2);

        assert!(out.is_none());
        assert_eq!(r1, Range { min: 0, max: 6 });
    }
}
