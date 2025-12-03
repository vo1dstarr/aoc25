use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = part1(&input).unwrap();
    println!("Part 1: {}", result);

    let result = part2(&input).unwrap();
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut result = 0;
    for str_range in input.trim().split(',') {
        // println!("{:?}", str_range);
        let mut nums_iter = str_range.split('-');
        let start = nums_iter.next().unwrap().parse::<usize>().unwrap();
        let end = nums_iter.next().unwrap().parse::<usize>().unwrap();
        for n in start..=end {
            if invalid_id(n) {
                result += n;
            }
        }
    }
    Ok(result)
}

fn invalid_id(n: usize) -> bool {
    let s = n.to_string();
    if s.len() % 2 == 1 {
        //if odd, it has to be valid
        return false;
    }
    let (h1, h2) = s.split_at(s.len() / 2);
    for (c1, c2) in h1.chars().zip(h2.chars()) {
        if c1 != c2 {
            return false;
        }
    }
    true
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut result = 0;
    for str_range in input.trim().split(',') {
        // println!("{:?}", str_range);
        let mut nums_iter = str_range.split('-');
        let start = nums_iter.next().unwrap().parse::<usize>().unwrap();
        let end = nums_iter.next().unwrap().parse::<usize>().unwrap();
        for n in start..=end {
            if invalid_id_part2(n) {
                result += n;
            }
        }
    }
    Ok(result)
}

fn invalid_id_part2(n: usize) -> bool {
    let mut result = false;

    let s = n.to_string();
    for i in 1..=s.len() / 2 {
        if !s.len().is_multiple_of(i) {
            continue;
        }

        let mut start = 0;
        let mut end = i;
        let cmp = &s[start..end];

        let mut invalid = true;
        while end <= s.len() {
            if cmp != &s[start..end] {
                invalid = false;
                break;
            }
            start += i;
            end += i;
        }
        if invalid {
            // println!("{:?} is invalid with size {:?}", s, i);
            result = true;
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = part1(input);
        assert_eq!(result.unwrap(), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = part2(input);
        assert_eq!(result.unwrap(), 4174379265);
    }
}
