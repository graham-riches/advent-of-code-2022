use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
struct SensorPoint {
    s: (i32, i32),
    b: (i32, i32),
    db: i32,
}

impl SensorPoint {
    // Build point from vector
    fn from_vec(v: &Vec<i32>) -> Option<Self> {
        if v.len() < 4 {
            None
        } else {
            let s = (v[0], v[1]);
            let e = (v[2], v[3]);
            Some(
                Self {
                    s: s,
                    b: e,
                    db: SensorPoint::get_distance(&s, &e)
                }
            )
        }        
    }

    // Returns a tuple of min/max coordinates at each row intersection
    fn intersect_at_row(&self, r: i32) -> Option<(i32, i32)> {                
        let delta = SensorPoint::get_distance(&self.s, &(self.s.0, r));
        if delta > self.db {
            return None;
        }                
        Some((self.s.0 - (self.db - delta), self.s.0 + (self.db - delta)))
    }

    // Get taxicab distance
    fn get_distance(s: &(i32, i32), e: &(i32, i32)) -> i32 {
        (s.0 - e.0).abs() + (s.1 - e.1).abs()
    }
}


fn main() {
    let pairs = std::fs::read_to_string("input.txt").unwrap()
     .split("\n")
     .map(|x| parse_line(x))
     .collect::<Vec<_>>();
        
    println!("Part one: {:?}", part_one(&pairs, 2000000));
    let p2 = part_two(&pairs, 0, 4000000);
    println!("Part two: {:?}", p2.0 as u64 * 4000000 + p2.1 as u64);
}

// Parse a line into sensor point start + end + taxi cab distance
fn parse_line(s: &str) -> SensorPoint {
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();    
    let points = re.captures(s).unwrap().iter().skip(1)
     .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
     .collect::<Vec<_>>();    
    SensorPoint::from_vec(&points).unwrap()
}

// Gets a list or ranges of covered spaces in a row
fn get_spaces_covered_in_row(pairs: &Vec<SensorPoint>, row: i32) -> Vec<(i32, i32)> {
    // Get range of coordinates that the signal intersects target row
    let ranges = pairs.iter()
     .flat_map(|x| x.intersect_at_row(row))
     .collect::<Vec<_>>();

    // Get all beacons in target row
    let beacons_in_row = pairs.iter()
     .flat_map(|point| if point.b.1 == row { Some(point.b.0) } else { None })
     .collect::<HashSet<i32>>();

    // For each range, split it into sub ranges if it intersects with a beacon
    let mut new_ranges = Vec::new();    
    for range in ranges {
        let mut beacons = beacons_in_row.iter()
         .flat_map(|x| if x >= &range.0 && x <= &range.1 { Some(*x) } else { None })
         .collect::<Vec<_>>();        
        beacons.sort();        
        let mut p = (range.0, range.1);
        for beacon in beacons {                        
            let left = (p.0, beacon - 1);
            if left.1 > p.0 {
                new_ranges.push(left);
            }                        
            p.0 = beacon + 1;
        }        
        if p.0 <= p.1 {
            new_ranges.push(p);
        }        
    }

    // Scan all ranges and sum unique values
    new_ranges.sort();
    new_ranges
}

// Part one solution
fn part_one(pairs: &Vec<SensorPoint>, row: i32) -> i32 {    
    let ranges = get_spaces_covered_in_row(pairs, row);
    let mut count = 0;
    let mut min = i32::MIN;    
    for range in ranges {        
        let m = if range.0 > min { range.0 } else { min + 1 };        
        if m < range.1 {
            count += range.1 - m + 1;
            min = range.1
        }                
    }
    count
}

// Part two solution
fn part_two(pairs: &Vec<SensorPoint>, min: i32, max: i32) -> (i32, i32) {
    let beacons = pairs.iter()
     .map(|x| x.b)
     .collect::<HashSet<(i32, i32)>>();
    
    for y in min..=max {
        let ranges = get_spaces_covered_in_row(pairs, y);              
        let mut it = ranges.iter();
        let (left, mut right) = it.next().unwrap();
        if *left > min {
            for i in *left..=min {
                return (i, y);
            }
        }
        while let Some(next) = it.next() {
            if next.0 - right >= 2 {                
                for i in (right + 1)..next.0 {
                    let p = (i, y);
                    if !beacons.contains(&p) {
                        return (i, y);
                    }                    
                }
            }
            if next.0 < right {                
                if next.1 > right {
                    right = next.1;
                }
                continue;
            }
            right = next.1;
        }        
    }    
    (0, 0)
}