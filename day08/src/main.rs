// I *could* do what clippy wants, but I think a simple index is clearer in this context
#![allow(clippy::needless_range_loop)]

use std::{cell::RefCell, collections::HashSet, fs, str::FromStr};

use anyhow::{Ok, anyhow};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = part1(&input, 1000).unwrap();
    println!("Part 1: {}", result);

    //     let result = part2(&input).unwrap();
    //     println!("Part 2: {}", result);
}

fn part1(input: &str, iterations: usize) -> anyhow::Result<usize> {
    let points = input
        .lines()
        .map(|line| line.parse::<Point>().unwrap())
        .collect::<Vec<_>>();
    let mut distances = Vec::new();
    for (idx1, p1) in points.iter().enumerate() {
        for (idx2, p2) in points.iter().enumerate().skip(idx1 + 1) {
            // if idx1 == idx2 {
            //     continue;
            // }
            let distance = distance(p1, p2);
            distances.push((idx1, idx2, distance));
        }
    }
    distances.sort_unstable_by(|lhs, rhs| lhs.2.total_cmp(&rhs.2));
    let mut circuits: Vec<RefCell<HashSet<usize>>> = Vec::new();
    for i in 0..iterations {
        let (p1, p2, _) = distances[i];
        let mut p1_set = None;
        let mut p2_set = None;
        for (cidx, set) in circuits.iter().enumerate() {
            if set.borrow().contains(&p1) {
                p1_set = Some(cidx);
            }
            if set.borrow().contains(&p2) {
                p2_set = Some(cidx);
            }
            if p1_set.is_some() && p2_set.is_some() {
                break;
            }
        }
        match (p1_set, p2_set) {
            (None, None) => {
                let set = HashSet::from([p1, p2]);
                circuits.push(RefCell::new(set));
            }
            (None, Some(cidx)) => {
                circuits[cidx].borrow_mut().insert(p1);
            }
            (Some(cidx), None) => {
                circuits[cidx].borrow_mut().insert(p2);
            }
            (Some(idx1), Some(idx2)) => {
                if idx1 != idx2 {
                    circuits[idx1]
                        .borrow_mut()
                        .extend(circuits[idx2].borrow_mut().drain());
                }
            }
        }
    }
    circuits.sort_unstable_by_key(|set| set.borrow().len());
    // dbg!(&circuits);
    Ok(circuits
        .iter()
        .rev()
        .take(3)
        .fold(1, |acc, set| acc * set.borrow().len().max(1)))
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut point = Point::default();
        let mut iter = s.split(',');
        point.x = iter
            .next()
            .ok_or_else(|| anyhow!("bad number of cooridnates"))?
            .parse()?;
        point.y = iter
            .next()
            .ok_or_else(|| anyhow!("bad number of cooridnates"))?
            .parse()?;
        point.z = iter
            .next()
            .ok_or_else(|| anyhow!("bad number of cooridnates"))?
            .parse()?;
        Ok(point)
    }
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    f64::sqrt(
        f64::powi(f64::from(p1.x) - f64::from(p2.x), 2)
            + f64::powi(f64::from(p1.y) - f64::from(p2.y), 2)
            + f64::powi(f64::from(p1.z) - f64::from(p2.z), 2),
    )
}

fn part2(input: &str) -> anyhow::Result<usize> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        let result = part1(input, 10);
        assert_eq!(result.unwrap(), 40);
    }

    #[test]
    fn test_part2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        let result = part2(input);
        assert_eq!(result.unwrap(), panic!("unknown answer"));
    }
}
