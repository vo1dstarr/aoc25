use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = part1(&input).unwrap();
    println!("Part 1: {}", result);

    let result = part2(&input).unwrap();
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut part_iter = input.split("\n\n");
    let ranges = part_iter.next().unwrap();
    let available = part_iter.next().unwrap();
    let mut ranges_vec = Vec::new();
    for line in ranges.lines() {
        let mut range_iter = line.split('-');
        let start = range_iter.next().unwrap().parse::<usize>().unwrap();
        let end = range_iter.next().unwrap().parse::<usize>().unwrap();
        ranges_vec.push((start, end));
    }
    Ok(available
        .lines()
        .filter_map(|s| {
            let n = s.parse::<usize>().unwrap();
            id_in_fresh_ranges(&ranges_vec, n).then_some(n)
        })
        .count())
}

fn id_in_fresh_ranges(ranges: &[(usize, usize)], item: usize) -> bool {
    for range in ranges {
        if item >= range.0 && item <= range.1 {
            return true;
        }
    }
    false
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut part_iter = input.split("\n\n");
    let ranges = part_iter.next().unwrap();
    let mut ranges_vec = Vec::new();
    for line in ranges.lines() {
        let mut range_iter = line.split('-');
        let start = range_iter.next().unwrap().parse::<usize>().unwrap();
        let end = range_iter.next().unwrap().parse::<usize>().unwrap();
        ranges_vec.push((start, end));
    }
    prune_ranges(&mut ranges_vec);
    Ok(ranges_vec.iter().map(|(start, end)| end - start + 1).sum())
}

fn prune_ranges(ranges: &mut Vec<(usize, usize)>) {
    ranges.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
    let mut prunable = Vec::new();
    for (i, (start, _end)) in ranges.iter().enumerate().skip(1) {
        // +1 because ranges are inclusive
        if *start <= ranges[i - 1].1 + 1 {
            //overlaps previous range
            prunable.push(i);
        }
    }
    // dbg!(&ranges);
    let pruned = prunable.len();
    for i in prunable.into_iter().rev() {
        let end = ranges[i].1;
        if end > ranges[i - 1].1 {
            ranges[i - 1].1 = end;
        }
        ranges.remove(i);
    }
    // dbg!(&ranges);
    if pruned > 0 {
        prune_ranges(ranges);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let result = part1(input);
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_part2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let result = part2(input);
        assert_eq!(result.unwrap(), 14);
    }

    #[test]
    fn test_part2_redundant() {
        let input = "3-5
10-14
16-20
12-18
13-15

1
5
8
11
17
32
";
        let result = part2(input);
        assert_eq!(result.unwrap(), 14);
    }

    #[test]
    fn test_part2_redundant_annoying() {
        let input = "3-5
10-14
16-20
12-18
13-14

1
5
8
11
17
32
";
        let result = part2(input);
        assert_eq!(result.unwrap(), 14);
    }
}
