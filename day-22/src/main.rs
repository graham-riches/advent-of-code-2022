use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Rotation {
    Right,
    Left
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Rotation(Rotation),
    Move(i32)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Heading {
    Up,
    Right,
    Left,
    Down
}

#[derive(Debug, Clone)]
struct Grid {
    map: HashMap<(i32, i32), bool>,  // Bool is true if entry is blocked
    loc: (i32, i32),                 // Current location
    heading: Heading
}

#[derive(Debug, PartialEq, Eq)]
struct GridError;

// Implement the from string trait for the map/grid for easy parsing
impl FromStr for Grid {
    type Err = GridError;

    // Parse a Grid object from a string
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.split("\n")
        .zip(1..)
        .map(|(line, row)| line.chars()
           .zip(1..)
           .flat_map(|(c, column)| match c {
               '.' => Some(((row, column), false)),
               '#' => Some(((row, column), true)),
               _   => None
           })
           .collect::<Vec<_>>()
       )
       .fold(HashMap::new(), |mut m, v| {
           for (key, val) in v {
               m.insert(key, val);
           }
           m
       });

       // Starting location is lowest column index in row 1
       let start_col = map.keys()
        .filter(|(row, _)| *row == 1)
        .map(|(_, c)| *c)
        .min()
        .unwrap();

       Ok( Grid{ map: map, loc: (1, start_col), heading: Heading::Right })
    }    
}

impl Grid {
    // Apply all instructions
    fn apply_instructions(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            match instruction {
                Instruction::Move(x)     => self.apply_move(*x),
                Instruction::Rotation(r) => self.apply_rotation(*r),
            }
        }
    }

    // Apply a move instruction
    fn apply_move(&mut self, distance: i32) {        
        let mut squares = self.map.keys()
         .filter(|(r, c)| match self.heading {
            Heading::Right => *r == self.loc.0,
            Heading::Left  => *r == self.loc.0,
            Heading::Down  => *c == self.loc.1,
            Heading::Up    => *c == self.loc.1,
         })
         .map(|(r, c)| match self.heading {
            Heading::Right => *c,
            Heading::Left  => *c,
            Heading::Down  => *r,
            Heading::Up    => *r,
         })
         .collect::<Vec<_>>();
        squares.sort();                

        for _ in 0..distance {
            let mut next = match self.heading {
                Heading::Right => self.loc.1 + 1,
                Heading::Left  => self.loc.1 - 1,
                Heading::Down  => self.loc.0 + 1,
                Heading::Up    => self.loc.0 - 1,
            };

            // Check wrap arounds
            if next > squares[squares.len() - 1] {
                next = squares[0];
            } else if next < squares[0] {
                next = squares[squares.len() - 1];
            }

            // Move the distance, if blocked, stop
            let next_coord = if self.heading == Heading::Left || self.heading == Heading::Right {
                (self.loc.0, next)
            } else {
                (next, self.loc.1)
            };

            if *self.map.get(&next_coord).unwrap() {                
                break;
            }
            self.loc = next_coord;            
        }        
    }

    // Apply a rotation instruction
    fn apply_rotation(&mut self, r: Rotation) {        
        match self.heading {
            Heading::Up    => self.heading = if r == Rotation::Right { Heading::Right } else { Heading::Left  },
            Heading::Down  => self.heading = if r == Rotation::Right { Heading::Left  } else { Heading::Right },
            Heading::Left  => self.heading = if r == Rotation::Right { Heading::Up    } else { Heading::Down  },
            Heading::Right => self.heading = if r == Rotation::Right { Heading::Down  } else { Heading::Up    },
        }        
    }

    // Calculate the secret password
    fn get_password(&self) -> i32 {
        let mut p = match self.heading {
            Heading::Right => 0,
            Heading::Down  => 1,
            Heading::Left  => 2,
            Heading::Up    => 3
        };
        p += (1000 * self.loc.0) + (4 * self.loc.1);
        p
    }
}




fn main() {
    let instructions = instructions_from_string(&std::fs::read_to_string("instructions.txt").unwrap());     
    let mut grid = Grid::from_str(&std::fs::read_to_string("map.txt").unwrap()).unwrap();    

    grid.apply_instructions(&instructions);
    let password = grid.get_password();
    println!("Part one: {}", password);
}

// Parses the map instructions from a string
fn instructions_from_string(s: &str) -> Vec<Instruction> {
    s.replace("R", ":R:")
     .replace("L", ":L:")
     .split(":")
     .map(|x| match x {
       "R" => Instruction::Rotation(Rotation::Right),
       "L" => Instruction::Rotation(Rotation::Left),
       x   => Instruction::Move(x.parse::<i32>().unwrap())
     })
     .collect::<Vec<_>>()
}