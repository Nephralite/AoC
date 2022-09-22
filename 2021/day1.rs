// imported from my replit at https://replit.com/@Jade-Ellingworth/day1#src/main.rs

fn formatted_input() -> Vec<i32> {
    include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part_one() -> i32 {
    let input = formatted_input();
    let mut last = 999;
    let mut output = 0;
    input.iter().for_each(|i| {
        if i > &last {
            output += 1;
        }
        last = *i;
    });
    output
}

fn part_two() -> i32 {
    let input = formatted_input();
    let mut last = vec![999, 999, 999];
    let mut output = 0;
    input.iter().for_each(|i| {
        if i > &last[0] {
            output += 1;
        }
        last.push(*i);
        last.remove(0);
    });
    output
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
