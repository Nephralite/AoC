fn formatted_input() -> Vec<usize> { //formats to how many calories on each elf
    include_str!("../input.txt")
        .split("\n\n") //split by elf
        .map(|x| {
            x.lines() //split into individual numbers
                .map(|n| n.parse::<usize>().unwrap()) //make numbers into numbers
                .sum::<usize>() //then sum them
        })
        .collect()
}

fn main() {
    let mut inp = formatted_input();
    inp.sort(); //for part_two
    println!("{:?}", inp.last().unwrap()); //part_one, originally used .iter().max() instead of last()
    println!("{:?}", inp.iter().rev().take(3).sum::<usize>()); //part_two
}
  
