use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let grid = Grid::new(input);
    let beam = grid.get_beam_start();
    let mut splitters = Splitters::default();

    process_line(beam, &grid, &mut splitters)
}

fn process_line(beam: Beam, grid: &Grid, splitters: &mut Splitters) -> usize {
    let traversal = grid.traverse(beam);

    match traversal {
        Traversal::End => 1,
        Traversal::Space(new_beam) => process_line(new_beam, grid, splitters),
        Traversal::Splitter(l, r) => {
            if let Some(timelines) = splitters.get_timelines(&beam.position) {
                timelines
            } else {
                let timelines = process_line(l, grid, splitters) + process_line(r, grid, splitters);
                splitters.insert_timelines(beam.position, timelines);
                timelines
            }
        }
    }
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn new(input: &str) -> Self {
        Grid(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn get_beam_start(&self) -> Beam {
        Beam::new(&self.0[0])
    }

    fn traverse(&self, beam: Beam) -> Traversal {
        let Beam {
            position: Position { height, index },
        } = beam;

        match self.0.get(height).map(|row| row.get(index)).flatten() {
            Some('^') => {
                let (a, b) = beam.split();
                Traversal::Splitter(a, b)
            }
            Some('.') => Traversal::Space(beam.move_down()),
            None => Traversal::End,
            _ => unreachable!(),
        }
    }
}

enum Traversal {
    Splitter(Beam, Beam),
    Space(Beam),
    End,
}

#[derive(Default)]
struct Splitters(HashMap<Position, usize>);

impl Splitters {
    fn get_timelines(&self, position: &Position) -> Option<usize> {
        self.0.get(position).copied()
    }

    fn insert_timelines(&mut self, position: Position, timelines: usize) {
        self.0.insert(position, timelines);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Beam {
    position: Position,
}

impl Beam {
    fn new(row: &Vec<char>) -> Self {
        let (index, _) = row.iter().enumerate().find(|(_, c)| **c == 'S').unwrap();

        Self {
            position: Position { height: 1, index },
        }
    }

    fn split(self) -> (Beam, Beam) {
        let a = Beam {
            position: Position {
                height: self.position.height + 1,
                index: self.position.index - 1,
            },
        };
        let b = Beam {
            position: Position {
                height: self.position.height + 1,
                index: self.position.index + 1,
            },
        };

        (a, b)
    }

    fn move_down(self) -> Beam {
        Beam {
            position: Position {
                height: self.position.height + 1,
                index: self.position.index,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    height: usize,
    index: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test() {
        let input = include_str!("../test-input.txt");

        let output = run(input);

        assert_eq!(output, 40);
    }
}
