//from https://replit.com/@Jade-Ellingworth/day8#src/main.rs

use std::iter::repeat;

fn formatted_input() -> Vec<&'static str> {
    let mut text: Vec<&str> = include_str!("../input.txt").split_whitespace().collect();
    text.retain(|&x| x != "|");
    text
}

fn part_one() {
    let input = formatted_input();
    let sections: Vec<&[&str]> = input.chunks(14).collect();
    let mut count = 0;
    let lengths = vec![2, 3, 4, 7];
    for section in &sections {
        for i in 10..14 {
            if lengths.contains(&section[i].len()) {
                count += 1;
            }
        }
    }
    println!("{}", count)
}

fn solve_section(section: &[&str]) -> usize {
    let mut solved: Vec<String> = repeat(String::new()).take(10).collect();
    let mut answer = 0;
    for digit in &section[0..10] {
        if digit.len() == 2 {
            //1 is the only value of len 2
            solved[1] = sort_string(digit);
        }
        if digit.len() == 3 {
            //7 is the only value of len 3
            solved[7] = sort_string(digit);
        }
        if digit.len() == 4 {
            //4 is the only value of len 4
            solved[4] = sort_string(digit);
        }
        if digit.len() == 7 {
            //8 is the only value of len 7
            solved[8] = sort_string(digit);
        }
    } // given this:
    let remains: Vec<&&str> = section[0..10]
        .iter()
        .filter(|n| !solved.contains(&sort_string(n)))
        .collect(); //collected due to borrowing
    for digit in remains {
        if digit.len() == 6 && overlap(digit, &solved[7]) == 2 {
            //of length 6, only 6 overlaps 7 twice / doesn't contain 7,
            solved[6] = sort_string(digit);
        } else if digit.len() == 6 && overlap(digit, &solved[4]) == 4 {
            //9 contains 4,
            solved[9] = sort_string(digit);
        } else if digit.len() == 6 {
            //and the only remaining len 6 is 0
            solved[0] = sort_string(digit);
        } else if overlap(digit, &solved[1]) == 2 {
            //of the length 5, (so all !len 6), only 3 contains 1
            solved[3] = sort_string(digit);
        } else if overlap(digit, &solved[4]) == 3 {
            //5 and 3 both overlap 3 with 4, but 5 is already solved above
            solved[5] = sort_string(digit);
        } else {
            //leaving only 2 left
            solved[2] = sort_string(digit);
        }
    } // so having decoded everything
    for digit in &section[10..] {
        answer = answer * 10
            + solved
                .iter()
                .position(|n| *n == sort_string(digit))
                .unwrap()
    }
    answer
}

fn overlap(a: &str, b: &String) -> usize {
    //number of chars in common
    a.chars().filter(|&n| b.contains(n)).count()
}

fn sort_string(str: &str) -> String {
    //removes anagram issues
    let mut temp: Vec<char> = str.chars().collect();
    temp.sort();
    temp.into_iter().collect()
}

fn part_two() {
    let input = formatted_input();
    let sections: Vec<&[&str]> = input.chunks(14).collect();
    let mut count = 0;
    for section in &sections {
        count += solve_section(section);
    }
    println!("{}", count);
}

fn main() {
    part_one();
    part_two();
}
