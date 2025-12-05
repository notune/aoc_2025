fn main() {
    let input = include_str!("../input");
//     let input = "3-5
// 10-14
// 16-20
// 12-18

// 1
// 5
// 8
// 11
// 17
// 32
// ";
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let (fresh_ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let mut num_valid_ingredients = 0;
    for ingredient_str in ingredients_str.lines() {
        let ingredient: u64 = ingredient_str.parse().unwrap();
        for fresh_range in fresh_ranges_str.lines() {
            let (b, e) = fresh_range.split_once('-').unwrap();
            let begin: u64 = b.parse().unwrap();
            let end: u64 = e.parse().unwrap();
            if ingredient >= begin && ingredient <= end {
                num_valid_ingredients += 1;
                break;
            }
        }
    }
    println!("Part 1: {}", num_valid_ingredients);
}

fn part2(input: &str) {
    let mut num_ids = 0;
    let (fresh_ranges_str, _) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(u64, u64)> = fresh_ranges_str.lines().map(|range| (range.split_once('-').unwrap().0.parse::<u64>().unwrap(), range.split_once('-').unwrap().1.parse::<u64>().unwrap())).collect();
    loop {
        let prev_len = ranges.len();
        let mut i = 0;
        while i < ranges.len() {
            let mut i2 = 0;
            let mut incr = true;
            while i2 < ranges.len() {
                let (begin, end) = ranges[i];
                let (begin2, end2) = ranges[i2];
                if begin == begin2 && end == end2 && i == i2 {
                    i2 += 1;
                }
                else if begin <= begin2 && end >= end2 {
                    ranges.remove(i2);
                    incr = false;
                }
                else if begin2 >= begin && begin2 <= end && end2 > end {
                    ranges[i] = (begin, end2);                
                    ranges.remove(i2);
                    incr = false;
                }
                else if end2 >= begin && end2 <= end && begin2 < begin {
                    ranges[i] = (begin2, end);                
                    ranges.remove(i2);
                    incr = false;
                } else {
                    i2 += 1;
                }
            }
            if incr {
                i += 1;
            }
        }
        if prev_len == ranges.len() {
            break;
        }
    }
    for range in ranges {
        let (b, e) = range;
        num_ids += e - b + 1;
    }
    println!("Part 2: {}", num_ids);
}
