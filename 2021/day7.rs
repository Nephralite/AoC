//from https://replit.com/@Jade-Ellingworth/day7#src/main.rs

fn formatted_input() -> Vec<i32> {
    include_str!("../input.txt")
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect()
}

fn fuel_to_pos_p1(pos: i32, crabs: &Vec<i32>) -> i32 {
    crabs.iter().map(|n| (n - pos).abs()).sum()
}

fn median(input: &Vec<i32>) -> i32 {
    let l = input.len();
    if l % 2 == 0 {
        (input[l / 2] + input[l / 2 - 1]) / 2
    } else {
        input[l / 2]
    }
}

fn part_one() {
    let mut vec = formatted_input();
    vec.sort();
    let m = median(&vec);
    println!("mode:{}, fuel:{}", m, fuel_to_pos_p1(m, &vec));
}

fn fuel_to_pos_p2(pos: i32, crabs: &Vec<i32>) -> i32 {
    crabs
        .iter()
        .map(|n| (0..(n - pos).abs() + 1).sum::<i32>())
        .sum() //to move 3 costs 1+2+3 which is the same as (1..4).sum (4 isnt included)
}

fn avg(input: &Vec<i32>) -> i32 {
    (input.iter().sum::<i32>() as f32 / input.len() as f32).floor() as i32
} // we are optimising for the triangle which is n*(n+1) /2
  //the avg optimises for n*n/2 which is always close enough that the answer is +-0.5 from the avg, and since we are only looking at integers that must always give the ceil or the floor
  //you could technically work out what constant to add but thats way harder than checking both
  //you could also use gradient descent which is what i was gonna do if i waas wrong about the average working

fn part_two() {
    let vec = formatted_input();
    let a = avg(&vec);
    let fuel_cost = fuel_to_pos_p2(a, &vec).min(fuel_to_pos_p2(a + 1, &vec));
    println!("avg based, fuel:{}", fuel_cost);
}

fn main() {
    part_one();
    part_two();
}
