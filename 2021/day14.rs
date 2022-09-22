use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Pair(char, char);

fn formatted_input() -> (HashMap<Pair, u64>, HashMap<Pair, Vec<Pair>>) {
    let mut input = include_str!("../input.txt").lines();
    let polymer: HashMap<Pair, u64> = input
        .next().unwrap() //"abcdef..."
        .chars().collect::<Vec<char>>() //[a,b,c,d,e,f...]
        .windows(2) // [ab, bc, cd ...]
        .map(|n| (Pair(n[0], n[1]), 1)) //all sequences in start are singular,
        //at least for my test and input
        .collect(); //{ab:1, bc:1, cd:1 ...}
    let pairs: HashMap<Pair, Vec<Pair>> = input
        .skip(1)
        .map(|n| n.split_once(" -> ").unwrap()) // "xy" -> z
        .map(|(inp, out)| {
            let x = inp.chars().next().unwrap();
            let y = inp.chars().nth(1).unwrap();
            let z = out.chars().next().unwrap();
            (Pair(x, y), vec![Pair(x, z), Pair(z, y)]) // xy : {xz, zy}
        })
        .collect();
    (polymer, pairs)
}

fn iterate(
    old_polymer: &HashMap<Pair, u64>,
    pairs: &HashMap<Pair, Vec<Pair>>,
) -> HashMap<Pair, u64> {
    let mut polymer = HashMap::new();
    old_polymer.into_iter().for_each(|(key, val)| {
        let new = pairs.get(&key).unwrap();
        let count = polymer.entry(new[0]).or_insert(0);
        *count += val;
        let count = polymer.entry(new[1]).or_insert(0);
        *count += val;
    });
    polymer
}

fn pairs_to_chars(polymer: HashMap<Pair, u64>) -> HashMap<char, u64> {
    let mut char_counts = HashMap::new();
    polymer.into_iter().for_each(|(key, val)| {
        let count = char_counts.entry(key.0).or_insert(1); //1 + so
                                                           //when div 2 odd rounds up and evens still round down
        *count += val;
        let count = char_counts.entry(key.1).or_insert(1);
        *count += val;
    });
    char_counts
}

fn repeated_iteration(step: u32) {
    let (mut polymer, pairs) = formatted_input();
    (0..step).for_each(|_i| polymer = iterate(&polymer, &pairs));
    let char_counts: Vec<u64> = pairs_to_chars(polymer)
        .into_values()
        .map(|n| n / 2)
        .collect();
    let ans = char_counts.iter().max().unwrap() - char_counts.iter().min().unwrap();
    println!("{}", ans)
}

fn main() {
    repeated_iteration(10);
    repeated_iteration(40);
}
