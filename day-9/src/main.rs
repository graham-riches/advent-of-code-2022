extern crate utilities;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32
}

fn main() {
    let moves = utilities::lines_from_file("input.txt").unwrap()
     .iter()
     .map(|x| x.split(" "))
     .map(|mut x| (x.next().unwrap().chars().next().unwrap(), 
                   x.next().unwrap().parse::<i32>().unwrap()))
     .collect::<Vec<(char, i32)>>();     

    // Part one
    let mut head = Position{ x: 0, y: 0 };
    let mut tail = Position{ x: 0, y: 0 };
    let mut positions_p1 = HashSet::new();

    for (direction, distance) in moves.iter() {
        for _ in 0..*distance {
            match direction {
                'R' => head.x += 1,
                'L' => head.x -= 1,
                'U' => head.y += 1,
                'D' => head.y -= 1,
                _   => (),
            }
            tail = update_tail_position(&head, &tail);            
            positions_p1.insert(tail);
        }        
    }    
    println!("Unique positions: {}", positions_p1.len());

    // Part two
    let mut knots = vec![Position{ x: 0, y: 0 }; 10];
    let mut positions_p2 = HashSet::new();

    for (direction, distance) in moves.iter() {        
        for _ in 0..*distance {
            match direction {
                'R' => knots[0].x += 1,
                'L' => knots[0].x -= 1,
                'U' => knots[0].y += 1,
                'D' => knots[0].y -= 1,
                _   => (),
            }
            for i in 1..knots.len() {                
                knots[i] = update_tail_position(&knots[i - 1], &knots[i]);
            }                        
            positions_p2.insert(knots[9]);
        }        
    }    
    println!("Unique positions: {}", positions_p2.len());

}

fn update_tail_position(h: &Position, t: &Position) -> Position {
    let dx = h.x - t.x;
    let dy = h.y - t.y;
    match (dx.abs(), dy.abs()) {        
        (2, 0) => Position{ x: t.x + dx.signum(), y: t.y },
        (0, 2) => Position{ x: t.x, y: t.y + dy.signum() },
        (2, 1) => Position{ x: t.x + dx.signum(), y: t.y + dy.signum() },
        (1, 2) => Position{ x: t.x + dx.signum(), y: t.y + dy.signum() },
        (2, 2) => Position{ x: t.x + dx.signum(), y: t.y + dy.signum() },
        _      => *t,
    }
}