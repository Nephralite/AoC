//from https://replit.com/@Jade-Ellingworth/day11#src/main.rs

fn formatted_input() -> Vec<u32> {
    include_str!("../input.txt")
        .chars()
        .filter(|&n| n.is_numeric())
        .map(|n| n.to_digit(10).unwrap())
        .collect()
}

fn flash(n: usize, list: &mut Vec<u32>) {
    let values = [-11, -10, -9, -1, 1, 9, 10, 11];
    for i in values {
        try_bump(n, n as isize + i, list)
    }
}

fn try_bump(flashing: usize, new: isize, list: &mut Vec<u32>) {
    if !(flashing % 10 == 9 && new % 10 == 0)
        && !(flashing % 10 == 0 && new % 10 == 9)
        && new >= 0
        && !(list.get(new as usize).is_none())
    {
        list[new as usize] += 1;
    }
}

fn iterate(mut list: Vec<u32>, mut count: u32) -> (Vec<u32>, u32) {
    list = list.iter().map(|n| n + 1).collect();
    let mut flashed = vec![];
    while (0..100)
        .filter(|n| list[*n] > 9 && !flashed.contains(n))
        .count()
        > 0
    {
        (0..100).for_each(|n| {
            if list[n] > 9 && !flashed.contains(&n) {
                flash(n, &mut list);
                flashed.push(n);
                count += 1;
            }
        })
    }
    (
        list.iter().map(|&n| if n > 9 { 0 } else { n }).collect(),
        count,
    )
}

fn parts_one_and_two() {
    let (mut list, mut count) = (formatted_input(), 0);
    let (mut step, mut ans_p1) = (0, 0);
    loop {
        (list, count) = iterate(list, count);
        step += 1;
        if step == 100 {
            ans_p1 = count;
        }
        if list.iter().sum::<u32>() == 0 {
            break;
        }
    }
    println!("{}\n{}", ans_p1, step);
}

fn main() {
    parts_one_and_two();
}
