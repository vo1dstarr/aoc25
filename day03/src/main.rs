use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = part1(&input).unwrap();
    println!("Part 1: {}", result);

    let result = part2(&input).unwrap();
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let result = input
        .lines()
        .map(|line| {
            let first = line[0..line.len() - 1] //can't be the last number
                .char_indices()
                .rev() //max_by_key: If several elements are equally maximum, the last element is returned.
                .max_by_key(|&(_idx, num)| num)
                .expect("iterator never empty");
            let second = line[first.0 + 1..line.len()]
                .char_indices()
                .max_by_key(|&(_idx, num)| num)
                .expect("iterator never empty");
            // dbg!(line);
            // dbg!(
            format!("{}{}", first.1, second.1)
                .parse::<usize>()
                .expect("is a valid positive number")
            // )
        })
        .sum();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let result = input
        .lines()
        .map(|line| {
            let mut str_builder = String::with_capacity(12);
            let mut start = 0;
            for i in (0..12).rev() {
                let end = line.len() - i;
                let max_found = line[start..end]
                    .char_indices()
                    .rev() //max_by_key: If several elements are equally maximum, the last element is returned.
                    .max_by_key(|&(_idx, num)| num)
                    .expect("iterator never empty");
                start += max_found.0 + 1; // the index of max_found is relative to slice, not the line, so we must +=
                str_builder.push(max_found.1);
            }
            // dbg!(line);
            // dbg!(
            str_builder
                .parse::<usize>()
                .expect("is a valid positive number")
            // )
        })
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";
        let result = part1(input);
        assert_eq!(result.unwrap(), 357);
    }

    #[test]
    fn test_part1_end() {
        let input = "117654321111198
";
        let result = part1(input);
        assert_eq!(result.unwrap(), 98);
    }

    #[test]
    fn test_part1_consec_middle() {
        let input = "117654398111111
";
        let result = part1(input);
        assert_eq!(result.unwrap(), 98);
    }

    #[test]
    fn test_part1_dup() {
        let input = "117654398111191
";
        let result = part1(input);
        assert_eq!(result.unwrap(), 99);
    }

    #[test]
    fn test_part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";
        let result = part2(input);
        assert_eq!(result.unwrap(), 3121910778619);
    }
}
