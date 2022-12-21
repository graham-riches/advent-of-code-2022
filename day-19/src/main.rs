#[derive(Debug, Clone, PartialEq, Eq)]
struct ResourceMonitor {
    robots: Vec<i32>,
    resources: Vec<i32>,
    to_build: Robot
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode
}

impl ResourceMonitor {
    fn new(ore_bot_cost: i32) -> Self {        
        let mut r = vec![0;4];
        r[0] = ore_bot_cost;
        Self {
            robots: vec![0; 4],
            resources: r,
            to_build: Robot::Ore
        }
    }

    fn increment(&mut self) {        
        self.resources[0] += self.robots[0];
        self.resources[1] += self.robots[1];
        self.resources[2] += self.robots[2];
        self.resources[3] += self.robots[3];
    }
}

fn get_max_geodes(mut state: ResourceMonitor, blueprint: &Blueprint, mut time_left: i32) -> i32 {    
    let mut built = false;
    while !built && time_left > 0 {
        match state.to_build {
            Robot::Ore      => {
                if state.resources[0] >= blueprint.ore {                        
                    state.resources[0] -= blueprint.ore;                                                
                    built = true;
                }
            },
            Robot::Clay     => {
                if state.resources[0] >= blueprint.clay {
                    state.resources[0] -= blueprint.clay;                        
                    built = true;
                }
            },
            Robot::Obsidian => {
                if state.resources[0] >= blueprint.obsidian.0 
                && state.resources[1] >= blueprint.obsidian.1 {                        
                    state.resources[0] -= blueprint.obsidian.0;
                    state.resources[1] -= blueprint.obsidian.1;                                                
                    built = true;
                }
            },
            Robot::Geode    => {
                if state.resources[0] >= blueprint.geode.0 && state.resources[2] >= blueprint.geode.1 {                        
                    state.resources[0] -= blueprint.geode.0;
                    state.resources[2] -= blueprint.geode.1;                                                
                    built = true;
                }                
            }
        }
        state.increment();
        time_left -= 1;
        
        if built {
            match state.to_build {
                Robot::Ore      => state.robots[0] += 1,
                Robot::Clay     => state.robots[1] += 1,
                Robot::Obsidian => state.robots[2] += 1,
                Robot::Geode    => state.robots[3] += 1,
            }                
        }            
    }
            
    let mut geodes = state.resources[3];    
    if time_left > 0 {
        for bot in [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode] {
            if bot == Robot::Ore && state.robots[0] >= blueprint.max_ore_per_turn
            || bot == Robot::Clay && state.robots[1] >= blueprint.obsidian.1
            || bot == Robot::Obsidian && state.robots[2] >= blueprint.geode.1 {
                continue;
            }

            if (bot == Robot::Obsidian && state.robots[1] == 0)
            || (bot == Robot::Geode    && state.robots[2] == 0) {
                continue;
            }
            let mut s = state.clone();            
            s.to_build = bot;
            let geode_count = get_max_geodes(s, blueprint, time_left);
            geodes = geodes.max(geode_count);
        }
    }
    geodes
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Blueprint {
    ore: i32,
    clay: i32,
    obsidian: (i32, i32),
    geode: (i32, i32),
    max_ore_per_turn: i32
}

impl Blueprint {
    fn from_vec(v: &Vec<i32>) -> Self {
        let ore_costs = vec![v[0], v[1], v[2], v[4]];

        Self {
            ore: v[0],
            clay: v[1],
            obsidian: (v[2], v[3]),
            geode: (v[4], v[5]),
            max_ore_per_turn: ore_costs.into_iter().max().unwrap_or(0),
        }
    }

    fn simulate_max_possible_geodes(&self, time: i32) -> i32 {           
        let m = ResourceMonitor::new(self.ore);        
        get_max_geodes(m, &self, time)
    }
}


fn main() {
    let blueprints = std::fs::read_to_string("input.txt").unwrap()
     .split("\n")
     .map(|line| {
        let i = line.find(":").unwrap();
        let mut s = line.to_string();
        s.replace_range(0..i, "");
        let s = s.replace(".", "");        
        let items = s.split(" ").flat_map(|x| x.parse::<i32>().ok()).collect::<Vec<_>>();      
        Blueprint::from_vec(&items)
     })
    .collect::<Vec<_>>();

    let p1 = blueprints.iter()
     .map(|b| b.simulate_max_possible_geodes(25))
     .zip(1..)
     .fold(0, |prod, (a, b)| prod + (a*b));

    println!("Part one: {}", p1);

    let mut p2 = 1;
    for i in 0..3 {
        p2 *= blueprints[i].simulate_max_possible_geodes(33);
    }
    println!("Part two: {}", p2)
}