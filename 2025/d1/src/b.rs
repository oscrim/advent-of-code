use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    L(usize),
    R(usize),
}

impl Rotation {
    fn rotate(&self, dial: &mut usize, counter: &mut usize) {
        match *self {
            Rotation::L(dist) => Self::rotate_left(dial, dist, counter),
            Rotation::R(dist) => Self::rotate_right(dial, dist, counter),
        }
    }

    fn rotate_left(dial: &mut usize, distance: usize, counter: &mut usize) {
        let full_rotations = distance / 100;
        let distance_remainder = distance % 100;

        let starting_dial = *dial;

        if distance_remainder > *dial {
            *dial = *dial + 100;
        }
        *dial = (*dial - distance_remainder) % 100;

        *counter += full_rotations;
        if *dial == 0 || (starting_dial < *dial && starting_dial != 0) {
            *counter += 1;
        }
    }

    fn rotate_right(dial: &mut usize, distance: usize, counter: &mut usize) {
        let full_rotations = distance / 100;
        let starting_dial = *dial;

        *dial = (*dial + distance) % 100;

        *counter += full_rotations;
        if starting_dial > *dial {
            *counter += 1;
        }
    }
}

pub fn run(input: &str) -> usize {
    let mut dial = 50_usize;
    let mut counter = 0_usize;

    input
        .lines()
        .into_iter()
        .map(|line| {
            let (dir, dist) = line.split_at(1);
            let dist: usize = FromStr::from_str(dist).unwrap();

            match dir {
                "R" => Rotation::R(dist),
                "L" => Rotation::L(dist),
                _ => unreachable!(),
            }
        })
        .for_each(|r| {
            r.rotate(&mut dial, &mut counter);
        });

    counter
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn run_test() {
        let input = include_str!("../test_input.txt");

        let output = run(input);

        assert_eq!(output, 6);
    }
}
