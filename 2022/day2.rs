use std::collections::HashMap;

fn formatted_input() -> Vec<&'static str> {
    include_str!("../input.txt").lines().collect()
}

fn part_one() {
    let inp = formatted_input();
    let points = HashMap::from([
        ("A X", 4), ("A Y", 8), ("A Z", 3),
        ("B X", 1), ("B Y", 5), ("B Z", 9),
        ("C X", 7), ("C Y", 2), ("C Z", 6),
    ]);
    println!("{:?}", inp.iter().map(|r| points.get(r).unwrap()).sum::<usize>());
}

fn part_two() {
    let inp = formatted_input();
    let points = HashMap::from([
        ("A X", 3), ("A Y", 4), ("A Z", 8),
        ("B X", 1), ("B Y", 5), ("B Z", 9),
        ("C X", 2), ("C Y", 6), ("C Z", 7),
    ]);
    println!("{:?}", inp.iter().map(|r| points.get(r).unwrap()).sum::<usize>());
}

fn main() {
    part_one();
    part_two();
}
