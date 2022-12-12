extern crate utilities;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    value: i32
}

struct Grid {
    grid: Vec<Vec<char>>,

}

impl Grid {
    // Create a new grid object
    fn new(g: Vec<Vec<char>>) -> Self {
        Self {
            grid: g
        }
    }

    // Helper function used to find coordinates of start and end
    // Note: this won't work on non-unique items
    fn find_unique_point(&self, v: char) -> Option<Point> {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] == v {
                    return Some(Point{ x: i as i32, y: j as i32, value: convert_char(v) });
                }
            }
        }
        None
    }

    // Get all points with a common elevation
    fn find_all_points_with_elevation(&self, elevation: char) -> Vec<Point> {
        let mut v = Vec::new();
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] == elevation {
                    v.push(Point{ x: i as i32, y: j as i32, value: convert_char(elevation) });
                }
            }
        }
        v
    }

    // Djikstras path finding
    fn find_shortest_path(&self, start: &Point, end: &Point) -> i32 {
        let mut min_costs: Vec<Vec<i32>> = vec![vec![i32::MAX; self.grid[0].len()]; self.grid.len()];
        min_costs[start.x as usize][start.y as usize] = 0;
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, start.x, start.y)));
        while let Some(Reverse((cost, x, y))) = queue.pop() {            
            if (x, y) == (end.x, end.y) { return cost; }

            if cost as i32 > min_costs[x as usize][y as usize] { continue; }

            for (r, c) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if self.grid.get(r as usize).and_then(|r1| r1.get(c as usize)).is_none() { continue; }  
                
                // Hackery to handle the 'S' and 'E' values
                let new_height = convert_char(self.grid[r as usize][c as usize]);                
                let current_height = convert_char(self.grid[x as usize][y as usize]);                
                if new_height - current_height <= 1 {
                    let new_cost = cost as i32 + 1;
                    if new_cost < min_costs[r as usize][c as usize] {                        
                        queue.push(Reverse((new_cost, r, c)));
                        min_costs[r as usize][c as usize] = new_cost;
                    }
                }
            }
        }
        i32::MAX
    }
}

// Helper function to map char values to ints
fn convert_char(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'S' => 1,
        'E' => 26,
        _   => -1
    }
}

fn main() {
    let grid: Vec<Vec<char>> = utilities::lines_from_file("input.txt").unwrap()
     .iter()
     .map(|x| x.chars().collect::<Vec<char>>())
     .collect();
        
    // Part one
    let g = Grid::new(grid);
    let start = g.find_unique_point('S').unwrap();
    let end = g.find_unique_point('E').unwrap();
    let path = g.find_shortest_path(&start, &end);
    println!("Shortest path is: {}", path);
    
    // Part two
    let points = g.find_all_points_with_elevation('a');
    let mut paths: Vec<i32> = points.iter()
     .map(|x| g.find_shortest_path(x, &end))
     .collect();
    paths.push(path);
    paths.sort();
    println!("Shortest path from any starting point: {}", paths[0]);
}