use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

pub fn run(input: &str, connections: usize) -> usize {
    let junction_boxes = parse_boxes(input);
    #[cfg(test)]
    println!("Boxes:\n{junction_boxes:#?}");

    let junction_box_distances = calculate_distances(&junction_boxes);
    #[cfg(test)]
    println!("Distances:\n{junction_box_distances:#?}");

    let mut juction_box_distance_vec: Vec<_> = junction_box_distances
        .into_iter()
        .map(|(connection, distance)| (connection, distance))
        .collect();

    juction_box_distance_vec.sort_unstable_by(|(_, a), (_, b)| a.total_cmp(b));

    let mut added_connections = 0;
    let mut circuts: Vec<HashSet<JunctionBoxId>> = Vec::new();

    for ((junc_a, junc_b), _) in juction_box_distance_vec {
        let circuit_a = circuts
            .iter()
            .enumerate()
            .find(|(_, set)| set.contains(&junc_a))
            .map(|(i, set)| (i, set.clone()));
        let circuit_b = circuts
            .iter()
            .enumerate()
            .find(|(_, set)| set.contains(&junc_b))
            .map(|(i, set)| (i, set.clone()));

        match (circuit_a, circuit_b) {
            (None, None) => {
                let mut set = HashSet::new();
                set.insert(junc_a);
                set.insert(junc_b);
                circuts.push(set);

                added_connections += 1;
            }
            (Some((i, _)), Some((j, _))) if i == j => {
                added_connections += 1;
            }
            (Some((i, set_a)), Some((j, set_b))) => {
                if i > j {
                    circuts.remove(i);
                    circuts.get_mut(j).unwrap().extend(set_a);
                } else {
                    circuts.remove(j);
                    circuts.get_mut(i).unwrap().extend(set_b);
                }
                added_connections += 1;
            }
            (None, Some((j, _))) => {
                circuts.get_mut(j).unwrap().insert(junc_a);
                added_connections += 1;
            }
            (Some((i, _)), None) => {
                circuts.get_mut(i).unwrap().insert(junc_b);
                added_connections += 1;
            }
        }

        if added_connections >= connections {
            break;
        }
    }

    #[cfg(test)]
    println!("Circuits:\n{circuts:#?}");

    let mut circut_lengths: Vec<_> = circuts.into_iter().map(|set| set.len()).collect();
    circut_lengths.sort();
    circut_lengths.reverse();

    let (a, rem) = circut_lengths.split_first().unwrap();
    let (b, rem) = rem.split_first().unwrap();
    let (c, _) = rem.split_first().unwrap();

    #[cfg(test)]
    println!("{a}*{b}*{c}");

    a * b * c
}

fn calculate_distances(boxes: &Vec<JunctionBox>) -> BTreeMap<(JunctionBoxId, JunctionBoxId), f64> {
    let mut distances = BTreeMap::new();

    for box1 in boxes {
        for box2 in boxes {
            if box1.id == box2.id || distances.contains_key(&(box2.id, box1.id)) {
                continue;
            }

            let distance = box1.distance(box2);

            distances.insert((box1.id, box2.id), distance);
        }
    }

    distances
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct JunctionBoxId(usize);

#[derive(Debug, Clone, Copy)]
struct JunctionBox {
    id: JunctionBoxId,
    x: f64,
    y: f64,
    z: f64,
}

impl JunctionBox {
    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

fn parse_boxes(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut split = line.split(',');

            JunctionBox {
                id: JunctionBoxId(id),
                x: FromStr::from_str(split.next().unwrap()).unwrap(),
                y: FromStr::from_str(split.next().unwrap()).unwrap(),
                z: FromStr::from_str(split.next().unwrap()).unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test() {
        let input = include_str!("../test-input.txt");

        let output = run(input, 10);

        assert_eq!(output, 40);
    }
}
