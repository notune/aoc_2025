
fn main() {
    let mut grid = include_str!("../input").lines().map(|l| l.as_bytes().to_vec()).collect();
//     let grid = "..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.".lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    part1(&grid);
    part2(&mut grid);
}


fn part1(grid: &Vec<Vec<u8>>) {
    let mut num_forklifts = 0;
    for r in 0i32..(grid.len() as i32) {
        for c in 0i32..(grid[0].len() as i32) {
            if grid[r as usize][c as usize] == b'@' {
                let mut num_papers = 0;
                for add_r in -1i32..2 {
                    for add_c in -1i32..2 {
                        if add_r == 0 && add_c == 0 {
                            continue;
                        }
                        let rowI = r + add_r;
                        let colI = c + add_c;
                        if rowI < 0 || rowI >= (grid[0].len() as i32) {
                            break;
                        }
                        if colI < 0 || colI >= (grid.len() as i32) {
                            continue;
                        }
                        if grid[rowI as usize][colI as usize] == b'@' {
                            num_papers += 1;
                        }
                    }
                }
                if num_papers < 4 {
                    num_forklifts += 1;
                }
            }
        }
    }
    println!("Part 1: {}", num_forklifts);
}

fn part2(grid: &mut Vec<Vec<u8>>) {
    let mut total_num_forklifts = 0;
    loop {
        let mut num_forklifts = 0;
        for r in 0i32..(grid.len() as i32) {
            for c in 0i32..(grid[0].len() as i32) {
                if grid[r as usize][c as usize] == b'@' {
                    let mut num_papers = 0;
                    for add_r in -1i32..2 {
                        for add_c in -1i32..2 {
                            if add_r == 0 && add_c == 0 {
                                continue;
                            }
                            let rowI = r + add_r;
                            let colI = c + add_c;
                            if rowI < 0 || rowI >= (grid[0].len() as i32) {
                                break;
                            }
                            if colI < 0 || colI >= (grid.len() as i32) {
                                continue;
                            }
                            if grid[rowI as usize][colI as usize] == b'@' {
                                num_papers += 1;
                            }
                        }
                    }
                    if num_papers < 4 {
                        num_forklifts += 1;
                        grid[r as usize][c as usize] = b'.';
                    }
                }
            }
        }
        if num_forklifts == 0 {
            break;
        }
        total_num_forklifts += num_forklifts;
    }
    println!("Part 2: {}", total_num_forklifts);
}
