use std::collections::VecDeque;
use std::sync::Arc;
use num::Num;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    calculate_new: Arc<dyn Fn(u64) -> u64>,    
    divisor: u64,
    a: usize,
    b: usize,
    modulus: Option<u64>,
    activity: usize,
}

impl Monkey {
    // Throw all items for a monkey, return a vec (next monkey, worry level)
    fn take_turn(&mut self) -> Vec<(usize, u64)> {
        let mut actions: Vec<(usize, u64)> = Vec::new();
        while let Some(item) = self.items.pop_front() {  
            self.activity += 1;          
            let mut new_val = (self.calculate_new)(item);
            match self.modulus {
                Some(m) => new_val %= m,
                _ => new_val /= 3
            }
            
            if new_val % self.divisor == 0 { 
                actions.push((self.a as usize, new_val));
            } else { 
                actions.push((self.b as usize, new_val));
            }
        }
        actions
    }
}


fn main() {
    let mut monkeys: Vec<Monkey> = std::fs::read_to_string("input.txt").unwrap()
     .split("\n\n")
     .map(|x| parse_monkey(x).unwrap())
     .collect();
    
    let p1 = get_monkey_business(monkeys.clone(), 20);
    println!("Part one: {}", p1);

    let modulus: u64 = monkeys.iter()
     .map(|x| x.divisor)
     .product();    
    for monkey in &mut monkeys {
        monkey.modulus = Some(modulus);
    }
    let p2 = get_monkey_business(monkeys, 10000);
    println!("Part one: {}", p2);

    
}

// Runs rounds and returns the monkey business score
fn get_monkey_business(mut monkeys: Vec<Monkey>, count: usize) -> usize {
    for _ in 0..count {
        for i in 0..monkeys.len() {
            let actions = monkeys[i].take_turn();
            for (to, value) in actions {
                monkeys[to].items.push_back(value);
            }
        }
    }
    let mut activity = monkeys.iter()
     .map(|x| x.activity)
     .collect::<Vec<usize>>();
    activity.sort_by(|a, b| b.cmp(a));
    activity[0] * activity[1]
}

// Strips a string prefix and returns a parsed result of the numeric value at the 
// end of the string
fn parse_trailing_number<T>(string: &str, prefix: &str) -> Option<T> 
where
    T: Num + std::str::FromStr
{
    string.trim_start()
     .strip_prefix(prefix)?
     .parse::<T>()
     .ok()
}

// Parses a single monkey from text, returns a monkey with a closure to handle new value calculation
fn parse_monkey(s: &str) -> Option<Monkey> {
    let t: Vec<&str> = s.split("\n").collect();
    let items = t[1].trim_start()
     .strip_prefix("Starting items: ")?
     .split(", ")
     .map(|x| x.parse::<u64>().unwrap())
     .collect::<VecDeque<u64>>();

    // Parse calculation parameters
    let args = t[2].trim_start()
     .strip_prefix("Operation: new = ")?
     .split(" ")     
     .collect::<Vec<&str>>();
    
    // Second argument to function not as string reference for lifetime issues
    let arg = match args[2] {
        "old" => None,
        _     => Some(args[2].parse::<u64>().unwrap())
    };

    // Create a closure to capture the "calculate new" logic
    let calculate_new_value: Arc<dyn Fn(u64) -> u64> = match (args[1], args[2]) {
        ("+", "old") => Arc::new(|x| x + x),
        ("*", "old") => Arc::new(|x| x * x),
        ("+", _)     => Arc::new(move |x| x + arg.unwrap()),
        ("*", _)     => Arc::new(move |x| x * arg.unwrap()),
        _            => Arc::new(|x| x)
    };
    
    // Parse arguments to function
    let div = parse_trailing_number::<u64>(t[3], "Test: divisible by ").unwrap();
    let a = parse_trailing_number::<u64>(t[4], "If true: throw to monkey ").unwrap();
    let b = parse_trailing_number::<u64>(t[5], "If false: throw to monkey ").unwrap();

    Some(Monkey {
        items: items,
        calculate_new: calculate_new_value,
        divisor: div,
        a: a as usize,
        b: b as usize,
        modulus: None,
        activity: 0,
    })
}

