use std::{collections::HashSet, str::FromStr};

pub fn run(input: &str) -> usize {
    let tiles: Vec<_> = input
        .lines()
        .into_iter()
        .map(|line| {
            let mut split = line.split(',');
            let x = FromStr::from_str(split.next().unwrap()).unwrap();
            let y = FromStr::from_str(split.next().unwrap()).unwrap();
            Tile { x, y }
        })
        .collect();

    let mut largest_area = 0;
    let mut rectangles = HashSet::new();

    for &tile1 in &tiles {
        for &tile2 in &tiles {
            if tile1 == tile2 {
                continue;
            }

            let rect = Rectangle::new(tile1, tile2);
            let area = rect.area();

            if rectangles.insert(rect) {
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }

    largest_area
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Tile {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rectangle {
    tile1: Tile,
    tile2: Tile,
}

impl Rectangle {
    fn new(tile1: Tile, tile2: Tile) -> Self {
        if tile1 < tile2 {
            Self { tile1, tile2 }
        } else {
            Self {
                tile1: tile2,
                tile2: tile1,
            }
        }
    }

    fn area(&self) -> usize {
        let x1 = self.tile1.x;
        let y1 = self.tile1.y;
        let x2 = self.tile2.x;
        let y2 = self.tile2.y;

        let dx = x1.abs_diff(x2) + 1;
        let dy = y1.abs_diff(y2) + 1;

        dx * dy
    }
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn test_run() {
        let input = include_str!("../test_input.txt");

        let output = run(input);

        assert_eq!(output, 50);
    }
}
