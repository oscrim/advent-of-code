use std::collections::HashSet;

pub fn run(input: &str) -> usize {
    let mut beam_indecies: HashSet<usize> = HashSet::default();
    let mut num_splits = 0;
    let mut lines = input.lines();

    let (index, _) = lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .find(|(_, val)| *val == 'S')
        .unwrap();

    beam_indecies.insert(index);

    for line in lines {
        process_line(&mut beam_indecies, line, &mut num_splits);
    }

    num_splits
}

fn process_line(beam_indecies: &mut HashSet<usize>, line: &str, num_splits: &mut usize) {
    let mut splits = Vec::default();

    for beam in beam_indecies.iter() {
        if line.get(*beam..*beam + 1) == Some("^") {
            splits.push(*beam);
        }
    }

    for split in splits {
        *num_splits += 1;
        split_beam(beam_indecies, split);
    }
}

fn split_beam(beam_indecies: &mut HashSet<usize>, index: usize) {
    beam_indecies.insert(index - 1);
    beam_indecies.insert(index + 1);
    beam_indecies.remove(&index);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn run_test() {
        let input = include_str!("../test-input.txt");

        let output = run(input);

        assert_eq!(output, 21);
    }

    #[test]
    fn test_split() {
        let mut beam_indecies: HashSet<usize> = HashSet::default();
        beam_indecies.insert(4);

        split_beam(&mut beam_indecies, 4);

        assert!(beam_indecies.contains(&3));
        assert!(beam_indecies.contains(&5));
        assert!(!beam_indecies.contains(&4));
    }

    #[test]
    fn test_split_input() {
        let line = ".......^.......";
        let mut num_splits = 0;
        let mut beam_indecies: HashSet<usize> = HashSet::default();
        beam_indecies.insert(7);

        process_line(&mut beam_indecies, line, &mut num_splits);

        assert_eq!(num_splits, 1);
        assert!(beam_indecies.contains(&6));
        assert!(beam_indecies.contains(&8));
        assert!(!beam_indecies.contains(&7));
    }
}
