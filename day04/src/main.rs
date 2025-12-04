use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = part1(&input).unwrap();
    println!("Part 1: {}", result);

    let result = part2(&input).unwrap();
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let matrix = make_matrix(input);
    Ok(num_access(&matrix))
}

type Matrix = Vec<Vec<u8>>;

fn make_matrix(input: &str) -> Matrix {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let mut v = Vec::new();
        for c in line.as_bytes() {
            v.push(*c);
        }
        matrix.push(v);
    }
    matrix
}

fn num_access(matrix: &Matrix) -> usize {
    accessable_items(matrix).len()
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut matrix = make_matrix(input);
    Ok(rec_num_access(&mut matrix, 0))
}

fn rec_num_access(matrix: &mut Matrix, acc: usize) -> usize {
    let to_be_removed = accessable_items(matrix);

    for (i, j) in &to_be_removed {
        matrix[*i][*j] = b'.';
    }

    if to_be_removed.is_empty() {
        return acc;
    }
    rec_num_access(matrix, acc + to_be_removed.len())
}

fn accessable_items(matrix: &Matrix) -> Vec<(usize, usize)> {
    fn neg(i: usize, _: usize) -> Option<usize> {
        i.checked_sub(1)
    }
    fn ident(i: usize, _: usize) -> Option<usize> {
        Some(i)
    }
    fn add(i: usize, max: usize) -> Option<usize> {
        let new = i + 1;
        if new < max {
            return Some(new);
        }
        None
    }
    let operations = [neg, ident, add];

    let mut to_be_removed = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if *item == b'.' {
                continue; //not paper
            }
            let i_max = matrix.len();
            let j_max = row.len();

            let mut addjacent_count = 0;
            for opp_i in operations.iter() {
                for opp_j in operations.iter() {
                    opp_i(i, i_max).map(|new_i| {
                        opp_j(j, j_max).map(|new_j| {
                            if matrix[new_i][new_j] == b'@' {
                                addjacent_count += 1;
                            }
                        })
                    });
                }
            }
            if addjacent_count < 5 {
                //5 because include self
                to_be_removed.push((i, j));
            }
        }
    }
    to_be_removed
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        let result = part1(input);
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        let result = part2(input);
        assert_eq!(result.unwrap(), 43);
    }
}
