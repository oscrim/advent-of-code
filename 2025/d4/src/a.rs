pub fn run(input: &str) -> usize {
    let column_len = input.lines().count();
    let row_len = input.lines().next().unwrap().chars().count();

    let mut board = vec![vec![0_i32; row_len]; column_len];

    for (i, line) in input.lines().enumerate() {
        //
        for (j, char) in line.chars().enumerate() {
            match char {
                '.' => {
                    *board.get_mut(i).unwrap().get_mut(j).unwrap() = -1;
                }
                '@' => {
                    for k in i.saturating_sub(1)..=i + 1 {
                        let Some(row) = board.get_mut(k) else {
                            continue;
                        };

                        for l in j.saturating_sub(1)..=j + 1 {
                            if i == k && j == l {
                                continue;
                            }

                            if let Some(val) = row.get_mut(l)
                                && *val != -1
                            {
                                //println!("{i}x{j} is adding to {k}x{j}");
                                *val += 1;
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    board.into_iter().fold(0, |acc, row| {
        acc + row.into_iter().fold(
            0,
            |acc, val| {
                if val >= 0 && val < 4 { acc + 1 } else { acc }
            },
        )
    })
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn run_test() {
        let input = include_str!("../test-input.txt");

        let output = run(input);

        assert_eq!(output, 13);
    }
}
