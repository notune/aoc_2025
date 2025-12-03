fn main() {
    let input = include_str!("../input").trim_end();
//     let input = "987654321111111
// 811111111111119
// 234234234234278
// 818181911112111";
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut sum_max = 0;
    for bank in input.lines() {
        let joltages: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut max_num = 0;
        for li in 0..joltages.len()-1 {
            for ri in li+1..joltages.len() {
                let num = joltages[li] * 10 + joltages[ri];
                if num > max_num {
                    max_num = num;
                }
            }
        }
        sum_max += max_num;
    }
    println!("Part 1: {sum_max}");
}

fn part2(input: &str) {
    let mut sum_max = 0;
    for bank in input.lines() {
        let joltages: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut num = 0;
        let mut max_index = 0;
        for n in (0..12).rev() {
            let mut digit: u64 = 0;
            for i in max_index..joltages.len()-(n as usize) {
                let joltage: u64 = joltages[i].into();
                if joltage > digit {
                    digit = joltage;
                    max_index = i;
                }
            }
            max_index += 1;
            num += u64::pow(10, n as u32) * digit;
        }
        sum_max += num;
    }
    println!("Part 2: {sum_max}");
}
