//from https://replit.com/@Jade-Ellingworth/day6#src/main.rs, removing an unused function

fn formatted_input() -> Vec<u32> {
    include_str!("../input.txt")
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect()
}

fn count<T: PartialEq>(i: T, vec: &Vec<T>) -> usize {
    //kinda overkill but haven't written generics in a while
    vec.iter().filter(|&n| n == &i).count()
}

fn to_count_vec(dumb_vec: Vec<u32>) -> Vec<u64> {
    let mut output: Vec<u64> = vec![];
    for i in 0..9 {
        output.push(count(i as u32, &dumb_vec) as u64);
    }
    output
}

fn smart_iteration(mut counts: Vec<u64>) -> Vec<u64> {
    // takes a vec of the count of each number, u64 just in case
    //vec n = vec n+1, exxcept vec 8 = vec 0 and vec 6 = vec 7 + vec 0 so
    counts.rotate_left(1);
    counts[6] += counts[8];
    counts
}

fn num_fish(days: u32) {
    let mut start: Vec<u64> = to_count_vec(formatted_input());
    for _i in 0..days {
        start = smart_iteration(start);
    }
    println!("{}", start.iter().fold(0, |acc, &x| acc + x));
}

fn main() {
    num_fish(80);
    num_fish(256);
}
