use std::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flowrate: i64,
    paths: Vec<String>
}

// Custom hash only on name
impl Hash for Valve {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Valve {
    // Parses a valve object from a string
    fn from_str(s: &str) -> Self {
        let chunks = s.split(" ").collect::<Vec<&str>>();
        let flow: i64 = chunks[4].split("=").skip(1).next().unwrap().strip_suffix(";").unwrap().parse::<i64>().unwrap();
        let mut paths = Vec::new();
        for path in chunks.iter().rev() {
            match *path {
                "valves" => break,
                "valve"  => break,
                _        => paths.push(path.strip_suffix(",").unwrap_or(path).to_string()),
            }
        }
        Self {
            name: chunks[1].to_string(),
            flowrate: flow,
            paths: paths
        }
    }
}

// Cave structure is a map of valve name and all valve objects
// plus the distance between all nodes in the graph
#[derive(Debug)]
struct Cave {
    map: HashMap<String, Valve>,
    distances: HashMap<(String, String), i64>
}


impl Cave {
    // Builds a cave out of map of valves
    fn new(map: HashMap<String, Valve>) -> Self {
        let mut cave = Self { map: map, distances: HashMap::new() };
        let distances = cave.get_shortest_path_between_nodes();
        cave.distances = distances;
        cave
    }

    // Get all path lengths between nodes in the graph
    fn get_shortest_path_between_nodes(&self) -> HashMap<(String, String), i64> {
        let mut map = HashMap::new();

        // Insert paths from self to self
        for key in self.map.keys() {
            map.insert((key.clone(), key.clone()), 0);
        }

        // Paths from each node to its adjacent nodes
        for (k, v) in self.map.iter() {
            for path in &v.paths { 
                map.insert((k.clone(), path.clone()), 1);
                map.insert((path.clone(), k.clone()), 1);
            }
        }

        // Paths between all nodes
        for (i, _) in self.map.iter() {
            for (j, _) in self.map.iter() {
                for (k, _) in self.map.iter() {
                    let dji = *map.get(&(j.clone(), i.clone())).unwrap_or(&i64::MAX);
                    let dik = *map.get(&(i.clone(), k.clone())).unwrap_or(&i64::MAX);
                    let djk = *map.get(&(j.clone(), k.clone())).unwrap_or(&i64::MAX);                
                    let dist = dji.saturating_add(dik);

                    if djk > dist {
                        map.insert((j.clone(), k.clone()), dist);
                    }
                }
            }
        }

        // Remove all self references
        for key in self.map.keys() {
            map.remove(&(key.clone(), key.clone()));
        }
        map
    }
}

fn main() {
    let valves = std::fs::read_to_string("input.txt").unwrap()
     .split("\n")
     .map(|line| Valve::from_str(line))
     .fold(HashMap::new(), |mut map, v| { map.insert(v.name.clone(), v); map});     
    
    let cave = Cave::new(valves); 

    // Ignore valves with flow-rate of zero as they are not worth visiting
    let valves_to_use = cave.map.values()
     .filter(|v| v.flowrate > 0)
     .map(|v| v.name.clone())
     .collect::<Vec<String>>();

    // Part one
    let mut paths = HashMap::new();
    for valve in &valves_to_use {
        let initial_distance = *cave.distances.get(&("AA".to_string(), valve.clone())).unwrap();
        find_best_path(valve, &valves_to_use, &cave, initial_distance, valve.to_string(), 0, HashSet::new(), &mut paths, 30);    
    }

    let p1 = paths.iter()
     .map(|(_, distance)| distance)
     .max()
     .unwrap();
    println!("Part one: {:?}", p1);


    // Part two
    let mut all_paths_with_help = HashMap::new();
    for valve in &valves_to_use {
        let initial_distance = *cave.distances.get(&("AA".to_string(), valve.clone())).unwrap();
        find_best_path(valve, &valves_to_use, &cave, initial_distance, valve.to_string(), 0, HashSet::new(), &mut all_paths_with_help, 26);    
    }

    // Find two largest paths that don't intersect at all
    let sets = all_paths_with_help.iter()
     .map(|(path, pressure)| {
        let path_set = path.split(",")
        .map(|x| x.to_string())
        .filter(|x| x.len() > 0)
        .collect::<HashSet<String>>();
        (path_set, *pressure)
     })
     .collect::<Vec<_>>();    

    let mut max_pressure = i64::MIN;    
    for (p1, c1) in sets.iter() {
        for (p2, c2) in sets.iter() {
            if p1.is_disjoint(p2) {
                if c1 + c2 > max_pressure {
                    max_pressure = c1 + c2;                    
                }
            }
        }
    }
    println!("Max pressure with help: {}", max_pressure);    
}

// Recursive approach to find best path - slow AF, but meh
fn find_best_path(
    start: &String,    
    valves: &[String],
    cave: &Cave,
    path: i64,
    path_taken: String,
    pressure_lost_total: i64,
    mut valves_used: HashSet<String>,
    all_paths: &mut HashMap<String, i64>,
    max_time: i64
)
{
    valves_used.insert(start.to_string());
    let v = cave.map.get(start).unwrap();
    let pressure_lost_current = (max_time - (path + 1)) * v.flowrate;
    all_paths.insert(path_taken.clone(), pressure_lost_total + pressure_lost_current);

    for key in valves {
        if !valves_used.contains(key) {            
            let d = *cave.distances.get(&(start.to_string(), key.clone())).unwrap();
            if d + path < max_time {
                let mut path_str = path_taken.clone();
                path_str.push(',');
                path_str.push_str(key.as_str());
                                
                find_best_path(
                    key,
                    valves,
                    cave,
                    d + path + 1,
                    path_str,
                    pressure_lost_total + pressure_lost_current,
                    valves_used.clone(),
                    all_paths,
                    max_time);
            }
        }
    }
}
