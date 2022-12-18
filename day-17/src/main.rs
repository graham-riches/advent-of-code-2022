use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct Rock {    
    points: Vec<(i64, i64)>, // [(X,Y)]
}

#[derive(Debug)]
struct CacheKey {
    rock_index: usize,
    wind_index: usize,
    set: HashSet<(i64, i64)>
}

// Custom hashing function for CacheKey
impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rock_index.hash(state);
        self.wind_index.hash(state);
    }
}

// Custom equality check for CacheKey: Sets must have equal items,
// and indices must all be the same
impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        if self.rock_index != other.rock_index 
        || self.wind_index != other.wind_index
        || self.set.len() != other.set.len() {
            false
        } else {                    
            for p in self.set.iter() {
                if !other.set.contains(p) {
                    return false;
                }
            }
            true
        }
    }
}

impl Eq for CacheKey {}


fn main() {
    let jets = std::fs::read_to_string("input.txt").unwrap()
     .chars()
     .collect::<Vec<_>>();
    
    let rocks = vec![
        Rock{ points: vec![(0,0), (1,0), (2,0), (3,0)] },
        Rock{ points: vec![(1,0), (0,1), (1,1), (2,1), (1, 2)] },
        Rock{ points: vec![(0,0), (1,0), (2,0), (2,1), (2,2)] },
        Rock{ points: vec![(0,0), (0,1), (0,2), (0,3)] },
        Rock{ points: vec![(0,0), (1,0), (0,1), (1,1)] },
    ];
                
    let p1 = simulate(&rocks, &jets, 2022);
    println!("Part one: {}", p1);
    
    let p2 = simulate(&rocks, &jets, 1000000000000);
    println!("Part one: {}", p2);
}

// Simulate a number of cycles of rock fall - uses caching to detect repeat cycles
fn simulate(rocks: &Vec<Rock>, jets: &Vec<char>, cycles: usize) -> i64 {  
    let mut cache: HashMap<CacheKey, (usize, i64, i64, usize, HashSet<(i64, i64)>)> = HashMap::new();

    // Track only the open set of points available that can be accessed by any path from above
    let mut surface = (0..7).map(|x| (x, -1)).collect::<HashSet<_>>();
        
    let mut height = 0;
    let rocks_len = rocks.len();    
    let mut wind_index = 0;
    let mut i = 0;
    while i < cycles {

        let rock_index = i % rocks_len;        
        let rock = rocks[rock_index].clone();
        
        let key = CacheKey{ 
            rock_index: rock_index,
            wind_index: wind_index,
            set: normalize_surface_height(&surface)};        
        if cache.contains_key(&key) {                               
            let (count, h, dh, nwi, new_surface) = cache.get(&key).unwrap();                        
            let dc = i - count;
            let number_repeats = (cycles - i) / dc as usize;                                    
            if number_repeats > 0 {
                // Apply height update for all cycles + current cycle, increment index, continue looping
                height += (height - h) * number_repeats as i64;                                
                i += dc * number_repeats;

                // Minor optimization: Apply update for current index (non-cycle) and continue
                surface = new_surface.clone();                
                height += dh;
                i += 1;
                wind_index = *nwi;
                continue;                                       
            }
        }

        // Store initial wind index prior to simulation
        let start_wind = wind_index;

        // Simulate the rock fall
        let (nwi, new_surface) = simulate_rock_fall(&surface, &rock, wind_index, jets);
        wind_index = nwi;        

        // Get the change in height from this iteration
        let max_height = get_max_surface_height(&new_surface);        
        let max_height_previous = get_max_surface_height(&surface);        
        let dh = max_height - max_height_previous;
                
        // Get new surface topology
        let next_surface = find_surface_topology(&new_surface);
        
        // Insert new cache entry        
        let key = CacheKey{ 
            rock_index: rock_index,
            wind_index: start_wind,
            set: normalize_surface_height(&surface)};                       
        
        cache.insert(key, (i, height, dh, wind_index, next_surface.clone()));                

        // Reset surface for next iteration        
        height += dh;
        i += 1;
        surface = next_surface;                               
    }
    height  
}


// Simulate the falling of one rock onto the existing surface. Returns a tuple of: (new wind index, new surface)
fn simulate_rock_fall(surface: &HashSet<(i64, i64)>, rock: &Rock, mut wind_index: usize, wind: &Vec<char>) -> (usize, HashSet<(i64, i64)>) {
    let wind_length = wind.len();
    
    // Seed initial rock points
    let start_height = get_max_surface_height(&surface) + 1;                
    let mut positions = rock.points.iter()
        .map(|(x, y)| (x + 2, y + start_height + 3))
        .collect::<Vec<_>>();
                
    // Store existing surface that will be appended to with current rock fall
    let mut new_surface = surface.clone();

    loop {                                   
        // Next position of rock
        let next = match wind[wind_index] {
            '>' => positions.iter().map(|(x, y)| (x + 1, *y)).collect::<Vec<_>>(),
            '<' => positions.iter().map(|(x, y)| (x - 1, *y)).collect::<Vec<_>>(),
            _   => positions.clone(),
        };

        wind_index += 1;
        wind_index %= wind_length;

        // Check in bounds to see if movement valid or if collision occurred
        let next_set = next.iter().cloned().collect::<HashSet<_>>();
        let intersect = surface.intersection(&next_set).count() > 0;
        if !intersect && next.iter().all(|(x, _)| *x >= 0 && *x < 7) {
            positions = next;
        }

        // Apply downwards movement
        let down = positions.iter()
            .map(|(x, y)| (*x, y - 1))
            .collect::<Vec<_>>();

        let down_set = down.iter().cloned().collect::<HashSet<_>>();
        
        // Check intersections            
        if surface.intersection(&down_set).count() > 0 {
            // We hit something, so stop at last position
            for pos in positions.iter() {
                new_surface.insert(*pos);
            }
            // Spawn new rock
            break;
        } else {
            positions = down;
        }            
        
    }
    (wind_index, new_surface)
}   


// Get the max height of the current surface
fn get_max_surface_height(surface: &HashSet<(i64, i64)>) -> i64 {
    surface.iter()
     .map(|(_, y)| *y)
     .max()
     .unwrap_or(0)
}


// Normalizes a surface so that the minimum Y is always height=0
fn normalize_surface_height(surface: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let min = surface.iter().map(|(_, y)| *y).min().unwrap_or(0);
    surface.iter()
     .map(|(x, y)| (*x, *y - min))
     .collect::<HashSet<_>>()
}


// Finds the normalized set of points that are occupied that can be reached from the top of the current grid
fn find_surface_topology(surface: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    // Start 1 higher in y to scan down into the current surface
    let h = get_max_surface_height(surface) + 1;    
    let points = (0..7_i64).map(|x| (x, h)).collect::<Vec<_>>();
    let mut new_surface = HashSet::new();
    let mut explored = HashSet::new();
    for p in &points {
        explore_surface(*p, &surface, &mut explored, &mut new_surface);
    }
    normalize_surface_height(&new_surface)
}


// Recursively explore downwards to find available spaces for pieces and return as new set
fn explore_surface(point: (i64, i64), existing_surface: &HashSet<(i64, i64)>, explored: &mut HashSet<(i64, i64)>, new_surface: &mut HashSet<(i64, i64)>) {
    let explore = vec![(point.0, point.1 - 1), (point.0 - 1, point.1), (point.0 + 1, point.1)];
    for p in &explore {
        if p.0 < 0 || p.0 == 7 {
            continue;
        }
        if !explored.contains(&p) {
            explored.insert(*p);
            if existing_surface.contains(&p) {
                new_surface.insert(*p);                
            } else {
                explore_surface(*p, existing_surface, explored, new_surface);
            } 
        }
    }
}
