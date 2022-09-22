//from https://replit.com/@Jade-Ellingworth/day13#src/main.rs

use std::collections::HashSet;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Point(i32, i32);

fn fold(point: Point, dir: char, val: i32) -> Point {
    //a fold at x=5 is (5,0)
    let mut output = Point(point.0, point.1);
    if dir == 'x' {
        output.0 = val - (output.0 - val).abs();
    }
    if dir == 'y' {
        output.1 = val - (output.1 - val).abs();
    }
    output
}

struct Fold(char, i32);

fn to_fold(n: &str) -> Fold {
    //n is x=255
    let dir = n.chars().nth(0).unwrap();
    let val: i32 = n
        .split(|c: char| !c.is_numeric())
        .last()
        .unwrap()
        .parse()
        .unwrap();
    Fold(dir, val)
}

fn formatted_input() -> (Vec<Point>, Vec<Fold>) {
    let input: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    let points_raw: Vec<i32> = input[0]
        .split(|c: char| !c.is_numeric())
        .map(|n| n.parse().unwrap())
        .collect();
    let points: Vec<Point> = points_raw.chunks(2).map(|n| Point(n[0], n[1])).collect();
    let folds: Vec<Fold> = input[1]
        .replace("fold along ", "")
        .lines()
        .map(|n| to_fold(n))
        .collect(); //x=325
    (points, folds)
}

fn part_one() {
    let (mut points, folds) = formatted_input();
    points = points
        .iter()
        .filter(|n| {
            if folds[0].0 == 'x' {
                n.0 != folds[0].1
            } else {
                n.1 != folds[0].1
            }
        })
        .map(|&n| fold(n, folds[0].0, folds[0].1))
        .collect();
    let dedup: HashSet<Point> = points.into_iter().collect();
    println!("{}", dedup.len());
}

fn part_two() {
    let (mut points, folds) = formatted_input();
    for f in folds {
        points = points
            .iter()
            .filter(|n| if f.0 == 'x' { n.0 != f.1 } else { n.1 != f.1 })
            .map(|&n| fold(n, f.0, f.1))
            .collect();
        points = points
            .into_iter()
            .collect::<HashSet<Point>>()
            .into_iter()
            .collect(); //removes duplicates by converting into hash set and back
    }
    let mut grid = vec![vec!['.'; 40]; 6];
    points
        .iter()
        .for_each(|n| grid[n.1 as usize][n.0 as usize] = '#');
    println!("{:?}", grid)
}

fn main() {
    part_one();
    part_two();
}
