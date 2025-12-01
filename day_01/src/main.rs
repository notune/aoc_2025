
fn main() {
    let input = include_str!("../input");
//     let input = "L68
// L30
// R48
// L5
// R60
// L55
// L1
// L99
// R14
// L82";
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut num_0s = 0;
    let mut dial: i32 = 50;
    for line in input.lines() {
        let dir = &line[0..1];
        let num: i32 = (&line[1..]).parse().unwrap();
        if dir == "L" {
            dial -= num;
        } else if dir == "R" {
            dial += num;
        }
        dial = dial.rem_euclid(100);
        if dial == 0 {
            num_0s += 1;
        }
    }
    println!("Solution: {}", num_0s)
}

fn part2(input: &str) {
    let mut num_0s = 0;
    let mut dial: i32 = 50;
    for line in input.lines() {
        let dir = &line[0..1];
        let num: i32 = (&line[1..]).parse().unwrap();
        if dir == "L" {
            let mut sub = dial;
            if dial == 0 {
                sub = 100;
            }
            num_0s += ((100 - sub) + num) / 100;
            dial -= num;
        } else if dir == "R" {
            num_0s += (dial + num) / 100;
            dial += num;
        }
        dial = dial.rem_euclid(100);
    }
    println!("Solution: {}", num_0s)
}
