extern crate utilities;
use itertools::Itertools;
pub mod range;
use crate::range::Range;

fn main() {    
    let input: Vec<(Range, Range)> = utilities::lines_from_file("input.txt")
     .unwrap()
     .into_iter()
     .map(|x| parse_line(&x))
     .collect();
    
    let p1 = input.iter()
     .map(|(x, y)| x.contains(&y) || y.contains(&x))
     .filter(|x| *x)
     .count();
    
    println!("Part one: {}", p1);

    let p2 = input.iter()
     .map(|(x, y)| x.overlaps(&y))
     .filter(|x| *x)
     .count();
    
    println!("Part two: {}", p2);
}

// Convert a line to a tuple of ranges
fn parse_line(s: &str) -> (Range, Range) {
    s.split(",")
     .map(|x| Range::from_str(x).unwrap())
     .next_tuple()
     .unwrap()
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("1-2,3-4"), (Range::new(1,2), Range::new(3,4)));
}

