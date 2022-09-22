//from https://replit.com/@Jade-Ellingworth/day9#src/main.rs

fn formatted_input() -> Vec<u32> {
    include_str!("../input.txt")
        .chars()
        .filter(|n| n.is_numeric())
        .map(|n| n.to_digit(10).unwrap())
        .collect()
}

fn less_than(a: u32, b: Option<&u32>) -> bool {
    if b.is_none() {
        true
    } else {
        &a < b.unwrap()
    }
}

fn try_get(original: isize, a: isize, list: &Vec<u32>) -> Option<&u32> {
    // don't try and index negatives, duh
    if a.is_negative() {
        None
    }
    //right hand side boundary condition
    else if original % 100 == 99 && a % 100 == 0 {
        None
    }
    //left hand side boundary condition
    else if original % 100 == 0 && a % 100 == 99 {
        None
    }
    else {
        list.get(a as usize)
    }
}

fn part_one() -> Vec<isize> {
    // not super efficient but pretty concise
    let input = formatted_input();
    let min_points = (0..input.len() as isize)
        .filter(|&n| {
            less_than(input[n as usize], try_get(n, n + 1, &input))
                && less_than(input[n as usize], try_get(n, n - 1, &input))
                && less_than(input[n as usize], try_get(n, n + 100, &input))
                && less_than(input[n as usize], try_get(n, n - 100, &input))
        })
        .collect::<Vec<isize>>(); //for just part one you could remove this collect
    let ans = min_points
        .iter()
        .map(|&n| input[n as usize] + 1)
        .sum::<u32>();
    println!("{}", ans);
    min_points // makes part_two easier
}

fn check_height(
    current: isize,
    place: isize,
    visited: &Vec<isize>,
    list: &Vec<u32>,
    stack: &Vec<isize>,
) -> bool {
    if visited.contains(&place) {
        false
    } else if stack.contains(&place) {
        false
    } else {
        let a = try_get(current, place, list);
        if a.is_none() {
            false
        } else {
            *a.unwrap() != 9 as u32
        }
    }
}

fn basin_size(min_point: isize, list: &Vec<u32>) -> u32 {
    let (mut visited, mut stack): (Vec<isize>, Vec<isize>) = (vec![], vec![]);
    let mut count = 0;
    stack.push(min_point);
    while stack.len() != 0 {
        let current = stack.pop().unwrap();
        visited.push(current);
        count += 1;
        if check_height(current, current + 1, &visited, list, &stack) {
            stack.push(current + 1)
        } //kinda ugly but working function
        if check_height(current, current - 1, &visited, list, &stack) {
            stack.push(current - 1)
        }
        if check_height(current, current + 100, &visited, list, &stack) {
            stack.push(current + 100)
        }
        if check_height(current, current - 100, &visited, list, &stack) {
            stack.push(current - 100)
        }
    }
    count
}

fn part_two(min_points: Vec<isize>) {
    let input = formatted_input();
    let mut basins: Vec<u32> = vec![];
    for point in min_points {
        basins.push(basin_size(point, &input));
    }
    basins.sort();
    println!("{:?}", &basins[basins.len() - 3..].iter().product::<u32>())
}

fn main() {
    part_two(part_one());
}
