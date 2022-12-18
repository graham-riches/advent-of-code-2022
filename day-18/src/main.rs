use std::collections::{HashMap, HashSet, VecDeque};


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32, 
    y: i32,
    z: i32
}

impl Point {
    fn from_vec(v: &Vec<i32>) -> Self {
        Point{ x: v[0], y: v[1], z: v[2] }
    }

    // Get all neighbour points that are in the "grid/arena"
    fn get_neighbours_in_bounds(&self, min: &Point, max: &Point) -> Vec<Point> {
        let mut n = Vec::new();
        n.push(Point{ x: self.x - 1, y: self.y, z: self.z } );
        n.push(Point{ x: self.x + 1, y: self.y, z: self.z } );
        n.push(Point{ x: self.x, y: self.y + 1, z: self.z } );
        n.push(Point{ x: self.x, y: self.y - 1, z: self.z } );
        n.push(Point{ x: self.x, y: self.y, z: self.z + 1 } );
        n.push(Point{ x: self.x, y: self.y, z: self.z - 1 } );
        n.into_iter()
         .filter(|p| p.x >= min.x - 1 && p.x <= max.x + 1 
                  && p.y >= min.y - 1 && p.y <= max.y + 1 
                  && p.z >= min.z - 1 && p.z <= max.z + 1 
        ).collect::<Vec<_>>()
    }
}


fn main() {
    let coords: HashMap<Point, bool> = std::fs::read_to_string("input.txt").unwrap()
     .split("\n")
     .map(|line| {
        let items = line.split(",").map(|p| p.parse::<i32>().unwrap()).collect::<Vec<_>>();
        Point::from_vec(&items)
     })
     .fold(HashMap::new(), |mut m, p| {
        m.insert(p, true);
        m
     });
    
    println!("Part one: {}", count_all_sides(&coords));
    println!("Part two: {}", count_exterior_faces(&coords));
}

// Get bounds of all points
fn get_bounds(points: &HashMap<Point, bool>) -> (Point, Point) {
    let x_min = points.keys().map(|p| p.x).min().unwrap();
    let y_min = points.keys().map(|p| p.y).min().unwrap();
    let z_min = points.keys().map(|p| p.z).min().unwrap();
    let x_max = points.keys().map(|p| p.x).max().unwrap();
    let y_max = points.keys().map(|p| p.y).max().unwrap();
    let z_max = points.keys().map(|p| p.z).max().unwrap();
    (Point{ x: x_min, y: y_min, z: z_min }, Point{ x: x_max, y: y_max, z: z_max })
}

// Counts all exterior faces by searching from outside the grid using a queue of points
fn count_exterior_faces(points: &HashMap<Point, bool>) -> usize {
    let mut count = 0;
    let (min, max) = get_bounds(points);

    // Find an initial seed point
    let start = Point{ x: min.x - 1, y: min.y - 1, z: min.z - 1 };
    
    let mut queue = VecDeque::new();
    queue.push_back(start.clone());

    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some(p) = queue.pop_front() {        
        // Check for other cubes in all directions
        for n in p.get_neighbours_in_bounds(&min, &max) {            
            if points.contains_key(&n) {
                count += 1;
            } else if !visited.contains(&n) {
                queue.push_back(n.clone());
                visited.insert(n);
            }
        }
    }
    count
}

// Counts all edges that don't intersect another cube
fn count_all_sides(points: &HashMap<Point, bool>) -> usize {
    let mut count = 0;
    let (min, max) = get_bounds(&points);
    for point in points.keys() {
        for neighbour in point.get_neighbours_in_bounds(&min, &max) {
            if !points.contains_key(&neighbour) {
                count += 1;
            }
        }
    }
    count
}
