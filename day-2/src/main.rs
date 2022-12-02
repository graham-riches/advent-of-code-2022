use std::fs;
use std::error::Error;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;     
    let strategy = input
        .split("\n")
        .map(|x| x.split(" ").next_tuple().unwrap())
        .collect::<Vec<(&str, &str)>>();
    
    let score_one = strategy.iter()
        .fold(0, |score, round| score + calculate_score_one(round));

    let score_two = strategy.iter()
    .fold(0, |score, round| score + calculate_score_two(round));

    println!("Part one: {}", score_one);
    println!("Part two: {}", score_two);
    Ok(())
}

fn calculate_score_one(round: &(&str, &str)) -> i32 {
    match round {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,        
        _ => 0
    }
}

fn calculate_score_two(round: &(&str, &str)) -> i32 {
    match round {
        ("A", "X") => 3,
        ("A", "Y") => 4,
        ("A", "Z") => 8,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 2,
        ("C", "Y") => 6,
        ("C", "Z") => 7,        
        _ => 0
    }
}