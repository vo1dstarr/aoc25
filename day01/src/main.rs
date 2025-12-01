use anyhow::{Error, Result, anyhow};
use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = part1(&input).unwrap();
    println!("Part 1 Zero Count: {}", result);
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let mut safe = Safe::new();
    for line in input.lines() {
        let turn = line.parse::<Turn>()?;
        safe.turn(turn);
    }
    Ok(safe.zero_count)
}

enum Turn {
    Left(i32),
    Right(i32),
}

impl FromStr for Turn {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let turn_char: char = s[..1].parse()?;
        let num: i32 = s[1..].parse()?;
        match turn_char {
            'L' => Ok(Turn::Left(num)),
            'R' => Ok(Turn::Right(num)),
            _ => Err(anyhow!("Turn Parse Error")),
        }
    }
}

#[derive(Debug)]
struct Safe {
    position: i32,
    zero_count: i32,
}

impl Safe {
    fn new() -> Safe {
        Safe {
            position: 50,
            zero_count: 0,
        }
    }

    fn turn(&mut self, t: Turn) {
        let add = match t {
            Turn::Left(n) => -n,
            Turn::Right(n) => n,
        };
        let new_position = self.position + add;
        // let mut crosses = new_position / 100;
        let remainder = new_position % 100;
        if remainder == 0 {
            self.zero_count += 1;
            self.position = remainder;
        } else if new_position <= 0 {
            // crosses += 1;
            // self.zero_count += crosses;
            self.position = 100 + remainder
        } else {
            // self.zero_count += crosses;
            self.position = remainder;
        }
        // println!("Safe: {:?}", self);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let result = part1(input);
        assert_eq!(result.unwrap(), 3);
    }
}
