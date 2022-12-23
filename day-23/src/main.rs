use std::collections::{HashSet, HashMap};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East
}

#[derive(Debug, Clone)]
struct Map {
    positions: HashSet<(i32, i32)>
}

#[derive(Debug, Copy, Clone)]
struct MapError;

// Implement the FromStr trait for parsing a map from string. This allows
// use of parse::<Map>(), which is convenient
impl FromStr for Map {
    type Err = MapError;

    // Parse a Map object from a string
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.split("\n")
         .zip(1..)
         .map(|(line, y)| line.chars()
         .zip(1..)
         .flat_map(move |(c, x)| match c {
             '#' => Some((x, y)),
             _   => None
         }))
         .flatten()
         .collect::<HashSet<_>>();
        Ok(Map{ positions: map })
    }
}

impl Map {
    // Motion simulation
    fn motion_sim(&mut self, rounds: Option<usize>) -> usize {        
        let check_directions = [
            Direction::North, Direction::South, 
            Direction::West, Direction::East];

        for round in 0.. {
            // First half of the sim
            let mut proposed = Vec::new();
            for (x, y) in self.positions.iter().cloned() {
                let neighbours = [
                    (x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y + 1),
                    (x + 1, y + 1), (x + 1, y), (x + 1, y - 1), (x, y - 1)];
                // If no neighbours occupied, do nothing
                if neighbours.iter().all(|x| !self.positions.contains(x)) {
                    continue;
                }
                // Otherwise, check neighbour directions in order
                for i in 0..check_directions.len() {
                    let index = (round + i) % check_directions.len();
                    let direction = check_directions[index];
                    if self.check_direction((x, y), direction) {
                        let next = Map::next_position((x, y), direction);
                        proposed.push(((x, y), next));
                        break;
                    }
                }
            }

            // Second half of the sim: drop all elves that are trying to move to the same location
            let mut m = HashMap::new();
            for (from, to) in proposed {
                if m.contains_key(&to) {
                    m.remove(&to);
                    continue;
                }
                m.insert(to, from);
            }
            // Duplicates removed, now move all valid entries in positions map
            for (to, from) in m.iter() {
                self.positions.remove(from);
                self.positions.insert(*to);
            }

            match rounds {
                Some(r) => if round == r - 1 { return r; },
                None => if m.len() == 0 { return round + 1; }
            }
        }
        0
    }

    // Check if motion in a direction is valid -> true if valid
    fn check_direction(&self, p: (i32, i32), direction: Direction) -> bool {
        let check_positions = match direction {
            Direction::North => [(p.0, p.1 - 1), (p.0 - 1, p.1 - 1), (p.0 + 1, p.1 - 1)],
            Direction::South => [(p.0, p.1 + 1), (p.0 - 1, p.1 + 1), (p.0 + 1, p.1 + 1)],
            Direction::West  => [(p.0 - 1, p.1), (p.0 - 1, p.1 - 1), (p.0 - 1, p.1 + 1)],
            Direction::East  => [(p.0 + 1, p.1), (p.0 + 1, p.1 - 1), (p.0 + 1, p.1 + 1)],
        };
        check_positions.iter().all(|p| !self.positions.contains(p))
    }

    // Helper to get next position for a given move direction
    fn next_position(p: (i32, i32), direction: Direction) -> (i32, i32) {
        match direction {
            Direction::North => (p.0, p.1 - 1),
            Direction::South => (p.0, p.1 + 1),
            Direction::West  => (p.0 - 1, p.1),
            Direction::East  => (p.0 + 1, p.1),
        }
    }

    // Count number of empty spaces in smallest rectangle containing all elves
    fn get_empty_tiles_in_container(&self) -> usize {
        let min_x = self.positions.iter().map(|(x, _)| *x).min().unwrap();
        let max_x = self.positions.iter().map(|(x, _)| *x).max().unwrap();
        let min_y = self.positions.iter().map(|(_, y)| *y).min().unwrap();
        let max_y = self.positions.iter().map(|(_, y)| *y).max().unwrap();
        let mut count = 0;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if !self.positions.contains(&(x, y)) {
                    count += 1
                }
            }
        }
        count
    }
}


fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = input.parse::<Map>().unwrap();

    // Part one
    let mut m1 = map.clone();
    m1.motion_sim(Some(10));
    println!("Part one: {:?}", m1.get_empty_tiles_in_container());

    // Part two
    let mut m2 = map.clone();
    let rounds = m2.motion_sim(None);
    println!("Part two: {}", rounds);
}