use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    L(usize),
    R(usize),
}

impl Rotation {
    fn rotate(&self, dial: &mut usize) {
        match *self {
            Rotation::L(dist) => Self::rotate_left(dial, dist % 100),
            Rotation::R(dist) => Self::rotate_right(dial, dist),
        }
    }

    fn rotate_left(dial: &mut usize, distance: usize) {
        if distance > *dial {
            *dial = *dial + 100;
        }

        *dial = (*dial - distance) % 100;
    }

    fn rotate_right(dial: &mut usize, distance: usize) {
        *dial = (*dial + distance) % 100;
    }
}

pub fn run(input: &str) -> usize {
    //let input = include_str!("test-input.txt");
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
            r.rotate(&mut dial);
            update_counter(&dial, &mut counter);
        });

    counter
}

fn update_counter(dial: &usize, counter: &mut usize) {
    if *dial == 0 {
        *counter += 1;
    }
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn run_test() {
        let input = include_str!("../test_input.txt");

        let output = run(input);

        assert_eq!(output, 3);
    }
}
