use std::hash::{Hash, Hasher};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flowrate: i32,
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
        let flow: i32 = chunks[4].split("=").skip(1).next().unwrap().strip_suffix(";").unwrap().parse::<i32>().unwrap();
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

fn main() {
    let valves = std::fs::read_to_string("sample.txt").unwrap()
     .split("\n")
     .map(|line| Valve::from_str(line))
     .fold(HashMap::new(), |mut map, v| { map.insert(v.name.clone(), v); map});     
    
    println!("Valves: {:?}", valves);
}


fn search(valves: &Vec<Valve>, current_flow_rate: i32, time_left: i32, visited: Vec<String>) -> i32 {
    
}


