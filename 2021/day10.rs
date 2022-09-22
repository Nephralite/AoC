//from https://replit.com/@Jade-Ellingworth/day10#src/main.rs

fn formatted_input() -> Vec<&'static str> {
    include_str!("../input.txt").lines().collect()
}

fn verify_line(line: &'static str) -> (usize, usize) {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if ['(', '<', '[', '{'].contains(&c) {
            stack.push(c);
        } else if stack.len() == 0 {
            return (syntax_score(c), 0);
        } else {
            let difference = c as isize - *stack.last().unwrap() as isize as isize;
            if difference == 1 || difference == 2 {
                stack.pop();
            } else {
                return (syntax_score(c), 0);
            }
        }
    }
    if stack.len() == 0 {
        (0, 0)
    }
    // if line is complete
    else {
        (0, stack_score(stack))
    } // if line is incomplete
}

fn syntax_score(s: char) -> usize {
    match s {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn parts_one_and_two() {
    let input: Vec<&str> = formatted_input();
    let (mut score_p1, mut score_p2) = (0, vec![]);
    for line in &input {
        let scores = verify_line(line);
        score_p1 += scores.0;
        score_p2.push(scores.1);
    }
    let mut score_p2_filtered: Vec<&usize> = score_p2.iter().filter(|&&n| n != 0).collect();
    score_p2_filtered.sort();
    println!("{}\n{}", score_p1, median(&score_p2_filtered));
}

fn stack_score(mut stack: Vec<char>) -> usize {
    stack.reverse();
    stack
        .iter()
        .map(|c| match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0, //shouldn't ever hit this but just in case
        })
        .fold(0, |acc, x| acc * 5 + x)
}

fn median(input: &Vec<&usize>) -> usize {
    let l = input.len();
    if l % 2 == 0 {
        (input[l / 2] + input[l / 2 - 1]) / 2
    } else {
        *input[l / 2]
    }
}

fn main() {
    parts_one_and_two();
}
