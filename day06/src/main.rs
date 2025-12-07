// I *could* do what clippy wants, but I think a simple index is clearer in this context
#![allow(clippy::needless_range_loop)]

use std::fs;

use anyhow::Ok;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = part1(&input).unwrap();
    println!("Part 1: {}", result);

    let result = part2(&input).unwrap();
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let matrix = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let opp_line_idx = matrix.len() - 1;
    let row_len = matrix[0].len();
    let mut result_sum = 0;
    for col in 0..row_len {
        let opp = matrix[opp_line_idx][col];
        let mut result = match opp {
            "*" => 1,
            "+" => 0,
            _ => panic!("bad input file"),
        };
        for row in 0..opp_line_idx {
            let num = matrix[row][col].parse::<usize>().unwrap();
            match opp {
                "*" => result *= num,
                "+" => result += num,
                _ => panic!("bad input file"),
            }
        }
        result_sum += result;
    }
    Ok(result_sum)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    //matrix of u8 this time
    let matrix = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let opp_line_idx = matrix.len() - 1;
    let row_len = matrix[0].len();
    let mut result_sum = 0;
    // find each opp and its range of columns
    for i in 0..row_len {
        let opp_item = matrix[opp_line_idx][i];
        if is_opp(opp_item) {
            let start = i;
            let mut end = row_len;
            for j in i + 1..row_len {
                if is_opp(matrix[opp_line_idx][j]) {
                    end = j;
                    break;
                }
            }
            // construct the verticle numbers
            let mut nums = Vec::new();
            for col in (start..end).rev() {
                let mut builder = String::new();
                for row in 0..opp_line_idx {
                    builder.push(matrix[row][col].into());
                }
                builder = builder.trim().to_string();
                if !builder.is_empty() {
                    let num = builder.parse::<usize>().unwrap();
                    nums.push(num);
                }
            }
            // do the math
            let init: usize = match opp_item {
                b'*' => 1,
                b'+' => 0,
                _ => panic!("bad input file"),
            };
            result_sum += nums.iter().fold(init, |acc, n| match opp_item {
                b'*' => acc * n,
                b'+' => acc + n,
                _ => panic!("bad input file"),
            });
        }
    }
    Ok(result_sum)
}

fn is_opp(byte: u8) -> bool {
    matches!(byte, b'*' | b'+')
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
        let result = part1(input);
        assert_eq!(result.unwrap(), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
        let result = part2(input);
        assert_eq!(result.unwrap(), 3263827);
    }
}
