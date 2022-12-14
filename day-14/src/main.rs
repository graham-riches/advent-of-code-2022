extern crate utilities;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    // Giant single expression monstrosity to build a hashmap of the scan data
    let rocks = std::fs::read_to_string("input.txt").unwrap()
     .split("\n")
     .map(|x| 
        x.split(" -> ")
         .flat_map(|y| utilities::parse_pair::<i32>(y, ","))
         .collect::<Vec<_>>())         
     .map(|paths| {
        let mut set = HashSet::new();
        let it = paths.iter();
        for (from, to) in it.tuple_windows() {
            set.insert(*from);
            set.insert(*to);
            if from.0 == to.0 {
                let range = if from.1 < to.1 { from.1..to.1 } else { to.1..from.1 };
                for r in range {
                    set.insert((from.0, r));
                }
            } else {
                let range = if from.0 < to.0 { from.0..to.0 } else { to.0..from.0 };
                for r in range {
                    set.insert((r, from.1));
                }
            }
        }
        set})     
      .fold(HashSet::new(), |mut set, s| { set.extend(s); set } );        
        
    let bottom = rocks.iter()
     .map(|(_, y)| y)
     .max()
     .unwrap();    
    
    println!("Part one: {}", simulate(&rocks, bottom + 1, |(_, y)| y >= *bottom));    
    println!("Part two: {}", simulate(&rocks, bottom + 2, |point| point == (500, 0) ));
}

// Sand falling simulation
fn simulate<F>(scan: &HashSet<(i32, i32)>, bottom: i32, exit: F) -> usize 
where F: Fn((i32, i32)) -> bool
{
    let mut filled = scan.clone();  
    let start = (500, 0);
    'outer: loop {
        let mut s = start;
        loop {            
            let falling_to = vec![(s.0, s.1 + 1), (s.0 - 1, s.1 + 1), (s.0 + 1, s.1 + 1)];
            let mut next_pos = s;
            for next in falling_to {
                if !filled.contains(&next) && next.1 < bottom {
                    next_pos = next;
                    break;
                }
            }            
            if next_pos == s {                
                filled.insert(s);
                break;
            } else {
                s = next_pos;
            }
            // Check exit condition in inner loop
            if exit(s) {                
                break 'outer;
            }
        }        
        // Check exit condition from outer loop
        if exit(s) {
            break;
        }
    }
    filled.len() - scan.len()
}