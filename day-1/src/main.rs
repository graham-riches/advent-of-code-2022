use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {    
    let mut calories: Vec<i32> = fs::read_to_string("input.txt")?
        .split("\n\n")
        .map(|x| { x.split("\n")
            .collect::<Vec<&str>>()
    }).map(|x| { x.iter().fold(0, |sum, cal| cal.parse::<i32>().unwrap() + sum) })
      .collect::<Vec<i32>>();    

    println!("Max calories: {}", calories.iter().max().unwrap());

    calories.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top_3 = calories.iter()
        .take(3)
        .fold(0, |sum, x| sum + x);
    println!("Top 3 sum: {}", top_3);
    Ok(())
}
