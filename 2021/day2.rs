//imported from https://replit.com/@Jade-Ellingworth/day2#src/main.rs

fn formatted_input() -> Vec<&'static str> {
    include_str!("../input.txt").lines().collect()
}

fn part_one() -> i32 {
    let input = formatted_input();
    let (mut xpos, mut ypos) = (0, 0);
    for i in input {
        let i: Vec<&str> = i.split(" ").collect();
        match i[..] {
            ["forward", x] => xpos += x.parse::<i32>().unwrap(),
            ["up", y] => ypos -= y.parse::<i32>().unwrap(),
            ["down", y] => ypos += y.parse::<i32>().unwrap(),
            _ => println!("malformed input!"),
        }
    }
    xpos * ypos
}

fn part_two() -> i32 {
    let input = formatted_input();
    let (mut xpos, mut ypos, mut aim) = (0, 0, 0);
    for i in input {
        let i: Vec<&str> = i.split(" ").collect();
        match i[..] {
            ["forward", x] => {
                xpos += x.parse::<i32>().unwrap();
                ypos += aim * x.parse::<i32>().unwrap();
            }
            ["up", z] => aim -= z.parse::<i32>().unwrap(),
            ["down", z] => aim += z.parse::<i32>().unwrap(),
            _ => println!("malformed input!"),
        }
    }
    xpos * ypos
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
