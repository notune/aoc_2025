fn main() {
    let input = include_str!("../input").trim_end();
    //let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut sum = 0;
    for range in input.split(',') {
        let (b, e) = range.split_once('-').unwrap();
        let begin: u64 = b.parse().unwrap();
        let end: u64 = e.parse().unwrap();
        let mut num_digits: u32 = b.len() as u32;
        let mut next_base10 = u64::pow(10, num_digits);
        for i in begin..end+1 {
            if i == next_base10 {
                num_digits += 1;
                next_base10 *= 10;
            }
            if num_digits % 2 != 0 {
                continue;
            }
            let first_half = (i % (u64::pow(10, num_digits))) / u64::pow(10, num_digits / 2);
            let second_half = i % u64::pow(10, num_digits / 2);
            if first_half == second_half {
                sum += i;
            }
        }
    }
    println!("Part 1: {}", sum);
}

fn part2(input: &str) {
    let mut sum = 0;
    for range in input.split(',') {
        let (b, e) = range.split_once('-').unwrap();
        let begin: u64 = b.parse().unwrap();
        let end: u64 = e.parse().unwrap();
        let mut num_digits: u32 = b.len() as u32;
        let mut next_base10 = u64::pow(10, num_digits);
        for i in begin..end+1 {
            if i == next_base10 {
                num_digits += 1;
                next_base10 *= 10;
            }
            'parts: 
            for n in 1..num_digits/2+1 { // n = size of parts
                if num_digits % n == 0 {
                    let mut mod_num = u64::pow(10, num_digits);
                    let mut div_num = u64::pow(10, num_digits-n);
                    let mut previous_part = (i % mod_num) / div_num;
                    for digit in 1..num_digits/n+1 { // iterate through parts
                        let current_part = (i % mod_num) / div_num;

                        if previous_part != current_part {
                            break;
                        }
                        if digit == num_digits/n {
                            sum += i;
                            break 'parts;
                        }

                        mod_num /= u64::pow(10, n);
                        div_num /= u64::pow(10, n);
                        previous_part = current_part;
                    }
                }
            }
        }
    }
    println!("Part 2: {}", sum);
}
