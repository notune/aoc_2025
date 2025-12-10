
fn main() {
    let input = include_str!("../input").trim_end();
//     let input = "123 328  51 64 
//  45 64  387 23 
//   6 98  215 314
// *   +   *   +  ";
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut matrix: Vec<_> = input.lines().map(|l| l.split_whitespace().collect::<Vec<_>>()).collect();
    matrix.reverse();
    let mut result = 0;
    for c in 0..matrix[0].len() {
        let operator: &str;
        let mut total: u64;
        if matrix[0][c] == "+" {
            operator = "+";
            total = 0;
        } else if matrix[0][c] == "*" {
            operator = "*";
            total = 1;
        } else {
            return
        }
        for r in 1..matrix.len() {
            if operator == "+" {
                total += matrix[r][c].parse::<u64>().unwrap();
            } else if operator == "*" {
                total *= matrix[r][c].parse::<u64>().unwrap();
            }
        }
        result += total;
    }
    println!("Part 1: {}", result);
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let mut operators: Vec<&str> = lines.next_back().unwrap().split_whitespace().collect();
    operators.reverse();
    let mut op_iter = operators.iter();
    let mut matrix: Vec<Vec<char>> = lines.map(|l| l.chars().rev().collect()).collect();
    matrix.reverse();
    let mut result: u64 = 0;
    let mut operator = *op_iter.next().unwrap();
    let mut intermediate_result: u64 = if operator == "*" { 1 } else { 0 };
    for c in 0..matrix[0].len() {
        let mut empty_row = true;
        let mut mul: u64 = 1;
        let mut num: u64 = 0;
        for r in 0..matrix.len() {
            let c = matrix[r][c];
            if c != ' ' {
                empty_row = false;
                num += mul * (c.to_digit(10).unwrap() as u64);
                mul *= 10;
            }
        }
        if empty_row {
            operator = *op_iter.next().unwrap();
            result += intermediate_result;
            intermediate_result = if operator == "*" { 1 } else { 0 };
        } else {
            if operator == "*" {
                intermediate_result *= num;
            } else if operator == "+" {
                intermediate_result += num;
            }
        }
    }
    
    result += intermediate_result;
    println!("Part 2: {}", result);
}
