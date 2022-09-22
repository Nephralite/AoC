//from https://replit.com/@Jade-Ellingworth/day3#src/main.rs

use std::iter::repeat;

fn txt_to_rotated_matrix(m_length: usize) -> Vec<String> {
    //using known length of data to easily rotate the raw string
    let contents = include_str!("../input.txt");
    let mut i = 0;
    let mut rotated: Vec<String> = repeat(String::new()).take(m_length).collect(); //newvec of empty strings
    for char in contents.chars() {
        if i < m_length {
            rotated[i].push(char);
            i += 1
        } else {
            i = 0
        } //nth character is /n so is discarded
    }
    rotated
}

fn most_common_bit(string: String) -> char {
    if 2 * string.chars().filter(|&n| n == '1').count() >= string.chars().count() {
        '1'
    } else {
        '0'
    }
    //if more than half of string is ones then 1 else 0, using multiplication on one side instead of dividing other by 2
}

fn part_one() -> u32 {
    let mut most_common = String::new();
    for line in txt_to_rotated_matrix(12) {
        most_common.push(most_common_bit(line));
    }
    let most_common = u32::from_str_radix(&most_common, 2).unwrap();
    most_common * (most_common ^ 0b111111111111) //the least common is the inverse of most common
} //works

fn part_two() -> usize {
    let mut input: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|n| usize::from_str_radix(n, 2).unwrap())
        .collect(); //cleanest import ever
    input.sort();
    oxygen(input.clone()) * co2(input)
} //works

fn oxygen(mut input: Vec<usize>) -> usize {
    let (mut i, mut answer) = (0, 0);
    while i < 12 {
        answer = answer * 2 + (input[input.len() / 2] >> (11 - i) & 1);
        input = input
            .into_iter()
            .filter(|n| (*n >> (11 - i)) == answer)
            .collect();
        i += 1;
        //println!("filtering by {:#0b}: {} remain",answer, input.len());
        if input.len() == 1 {
            break;
        }
    }
    *input.first().unwrap()
}

fn co2(mut input: Vec<usize>) -> usize {
    let (mut i, mut answer) = (0, 0);
    while i < 12 {
        answer = answer * 2 + (input[input.len() / 2] >> (11 - i) & 1) ^ 1; //middle value's bit is most common
        input = input
            .into_iter()
            .filter(|n| (*n >> (11 - i)) == answer)
            .collect();
        i += 1;
        //println!("filtering by {:#0b}: {} remain",answer, input.len());
        if input.len() == 1 {
            break;
        }
    }
    *input.first().unwrap()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
