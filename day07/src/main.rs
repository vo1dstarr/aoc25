// I *could* do what clippy wants, but I think a simple index is clearer in this context
#![allow(clippy::needless_range_loop)]

use std::{collections::HashSet, fs};

use anyhow::Ok;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = part1(&input).unwrap();
    println!("Part 1: {}", result);

    let result = part2(&input).unwrap();
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut beams = HashSet::new();
    let result = input
        .lines()
        .map(|line| {
            let mut splits = 0;
            for (idx, c) in line.char_indices() {
                match c {
                    'S' => {
                        beams.insert(idx);
                    }
                    '^' => {
                        let contained = beams.remove(&idx);
                        if contained {
                            splits += 1;
                            beams.insert(idx - 1);
                            beams.insert(idx + 1);
                        }
                    }
                    '.' => {}
                    _ => panic!("Bad input data"),
                }
            }
            splits
        })
        .sum();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut beams = Vec::new();
    input.lines().enumerate().for_each(|(line_num, line)| {
        for (idx, c) in line.char_indices() {
            if line_num == 0 {
                beams.push(0);
            }
            match c {
                'S' => {
                    beams[idx] += 1;
                }
                '^' => {
                    let num_beams = beams[idx];
                    if num_beams > 0 {
                        beams[idx] = 0;
                        beams[idx - 1] += num_beams;
                        beams[idx + 1] += num_beams;
                    }
                }
                '.' => {}
                _ => panic!("Bad input data"),
            }
        }
    });
    Ok(beams.iter().sum())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        let result = part1(input);
        assert_eq!(result.unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        let result = part2(input);
        assert_eq!(result.unwrap(), 40);
    }
}
