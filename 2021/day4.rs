//from https://replit.com/@Jade-Ellingworth/day4#src/main.rs

fn formatted_input() -> (Vec<usize>, Vec<usize>) {
    let mut input: Vec<&str> = include_str!("../input.txt").lines().collect(); //import as vec of lines
    let numbers: Vec<usize> = input[0].split(",").map(|n| n.parse().unwrap()).collect(); //take first line off for numbers to call
    input.remove(0);
    let card_data: Vec<usize> = input
        .join(" ")
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect(); //converts cards into single list of numbers
    (numbers, card_data)
}

fn part_one_and_two() {
    let (numbers, card_data) = formatted_input();
    let cards: Vec<&[usize]> = card_data.chunks(25).collect(); //split back into cards
    let mut bingo_times: Vec<usize> = vec![];
    for card in &cards {
        bingo_times.push(find_bingo_time(&numbers, card));
    } // enumerate gives (num of bingo card, num of numbers to finish), spent hours debugging cos had it wrong way round below :()
    let first_win_pos: (usize, &usize) = bingo_times
        .iter()
        .enumerate()
        .min_by_key(|(_idx, &val)| val)
        .unwrap();
    let part_one = calculate_score(&numbers[0..first_win_pos.1 + 1], &cards[first_win_pos.0]);
    let last_win_pos: (usize, &usize) = bingo_times
        .iter()
        .enumerate()
        .max_by_key(|(_idx, &val)| val)
        .unwrap();
    let part_two = calculate_score(&numbers[0..last_win_pos.1 + 1], &cards[last_win_pos.0]);
    println!("{}\n{}", part_one, part_two);
}

fn find_bingo_time(numbers: &Vec<usize>, card: &[usize]) -> usize {
    // works out how many numbers till a card wins
    let positions: Vec<usize> = card
        .iter()
        .map(|&n| numbers.iter().position(|&x| x == n).unwrap())
        .collect(); // gives position each number is called at
    let first_hor: &usize = positions
        .chunks(5)
        .map(|n| n.iter().max().unwrap())
        .min()
        .unwrap(); // gives the position of the first number to complete a horizontal row
    let mut max_of_cols: Vec<usize> = Vec::new();
    (0..5).for_each(|i| max_of_cols.push(max_of_col(&positions, i)));
    let first_ver: usize = *max_of_cols.iter().min().unwrap();
    first_ver.min(*first_hor)
}

fn max_of_col(card: &Vec<usize>, i: usize) -> usize {
    *card.chunks(5).map(|n| n.get(i).unwrap()).max().unwrap()
}

fn calculate_score(called: &[usize], card: &[usize]) -> usize {
    let remains: Vec<&usize> = card.iter().filter(|n| !called.contains(n)).collect();
    let sum: usize = remains.iter().fold(0, |sum, &x| sum + x);
    sum * called.last().unwrap()
}

fn main() {
    part_one_and_two();
}
