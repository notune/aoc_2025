use std::collections::{HashSet, HashMap};

fn main() {
    let input = include_str!("../input").trim_end();
//     let input = ".......S.......
// ...............
// .......^.......
// ...............
// ......^.^......
// ...............
// .....^.^.^.....
// ...............
// ....^.^...^....
// ...............
// ...^.^...^.^...
// ...............
// ..^...^.....^..
// ...............
// .^.^.^.^.^...^.
// ...............
// ";
    let matrix = input.lines().map(|l| l.chars().collect()).collect();
    part1(&matrix);
    part2(&matrix);
}

fn part1(matrix: &Vec<Vec<char>>) {
    let mut start_poss: HashSet<(usize,usize)> = HashSet::<(usize,usize)>::new();
    let mut split_poss: HashSet<(usize,usize)> = HashSet::<(usize,usize)>::new();
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            let symb = matrix[r][c];
            if symb == 'S' {
                start_poss.insert((r, c));
                break;
            }
        }
    }
    loop {
        let it_start_poss = start_poss.clone();
        for start_pos in &it_start_poss {
            let (r, c) = start_pos;
            for rd in r+1..matrix.len() {
                if matrix[rd][*c] == '^' {
                    start_poss.insert((rd, c-1));
                    start_poss.insert((rd, c+1));
                    start_poss.remove(start_pos);
                    split_poss.insert((rd, *c));
                    break;
                }
                if rd == matrix.len() - 1 {
                    start_poss.remove(start_pos);
                }
            }
        }
        if start_poss.len() == 0 {
            break;
        }
    }
    println!("Part 1: {}", split_poss.len());
}

fn num_paths(r: usize, c: usize, matrix: &Vec<Vec<char>>, num_paths_from_pos: &mut HashMap<(usize, usize), u64>) -> u64 {
    if num_paths_from_pos.contains_key(&(r,c)) {
        return *num_paths_from_pos.get(&(r,c)).unwrap();
    } else if matrix[r][c] == '^' {
        let num = num_paths(r, c-1, &matrix, num_paths_from_pos) + num_paths(r, c+1, &matrix, num_paths_from_pos);
        num_paths_from_pos.insert((r, c), num);
        return num;
    } else if r == matrix.len() - 1 {
        return 1;
    } else {
        return num_paths(r+1, c, matrix, num_paths_from_pos);
    }
}

fn part2(matrix: &Vec<Vec<char>>) {
    let mut start_r = 0;
    let mut start_c = 0;
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            let symb = matrix[r][c];
            if symb == 'S' {
                start_r = r;
                start_c = c;
                break;
            }
        }
    }
    println!("Part 2: {}", num_paths(start_r, start_c, matrix, &mut HashMap::new()));
}
