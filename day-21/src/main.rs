use std::collections::{HashMap};
extern crate utilities;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Expression {
    a: String,
    op: Op,
    b: String
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Entry {
    Number(i64),
    Expression(Expression)
}

fn main() {
    let map = utilities::lines_from_file("input.txt").unwrap()
     .iter()
     .map(|line| parse_line(line))
     .fold(HashMap::new(), |mut m, (k, e)| {
        m.insert(k, e);
        m
     });

    // Part one
    let p1 = get_expression_value(&map, "root");
    println!("Part one: {}", p1);

    // Part two    
    let mut input;
    let mut upper = 1;    
    
    // Binary search until upper bound is established.
    // This occurs when the check ah < bh changes from its initial value
    let (_, mut ah, mut bh) = check_root_equality(&map, upper);
    let initial_state = ah < bh;
    let mut new_state = initial_state;
    while new_state == initial_state {
        upper *= 2;
        (_, ah, bh) = check_root_equality(&map, upper);        
        new_state = ah < bh;
    }

    // Start lower bound at half the upper bound as this was the last tested location where the
    // state of the comparison was equal to the initial value
    let mut lower = upper / 2;

    // Now binary search between lower and upper to find the point where the inputs flipped        
    loop {
        input = (upper - lower) / 2 + lower;
        let (eq, a, b) = check_root_equality(&map, input);
        if eq { break; }
        let state = a < b;
        if state == initial_state {
            lower = input;
        } else {
            upper = input;
        }        
    }

    // Finally linear search between the two bounds to get the exact value
    for i in lower..upper {
        let (eq, _, _) = check_root_equality(&map, i);
        if eq { input = i; break; }
    }    
    println!("Part two: {}", input);
}

// Parse a line into an expression type
fn parse_line(s: &str) -> (String, Entry) {
    let items = s.split(" ").collect::<Vec<_>>();
    let key = items[0].strip_suffix(":").unwrap().to_string();
    if items.len() == 2 {
        (key, Entry::Number(items[1].parse::<i64>().unwrap()))
    } else {
        let a = items[1].to_string();
        let b = items[3].to_string();
        let exp = match items[2] {
            "+" => Expression{ a: a, op: Op::Add,      b: b },
            "-" => Expression{ a: a, op: Op::Subtract, b: b },
            "*" => Expression{ a: a, op: Op::Multiply, b: b },
            _   => Expression{ a: a, op: Op::Divide,   b: b },                
        };
        (key, Entry::Expression(exp))
    }
}

// Get the value for a particular entry
fn get_expression_value(map: &HashMap<String, Entry>, key: &str) -> i64 {    
    match map.get(key).unwrap() {
        Entry::Number(x) => *x,
        Entry::Expression(e) => {
            let a = get_expression_value(map, &e.a);
            let b = get_expression_value(map, &e.b);
            match e.op {
                Op::Add      => a + b,
                Op::Subtract => a - b,
                Op::Multiply => a * b,
                Op::Divide   => a / b
            }            
        }
    }
}

// For a given input, check the root equality condition
fn check_root_equality(map: &HashMap<String, Entry>, input: i64) -> (bool, i64, i64) {
    let mut m = map.clone();
    m.insert("humn".to_string(), Entry::Number(input));    
    match m.get("root").unwrap() {
        Entry::Number(_) => (false, i64::MIN, i64::MAX),
        Entry::Expression(e) => {
            let a = get_expression_value(&m, &e.a);
            let b = get_expression_value(&m, &e.b);
            (a == b, a, b)
        }
    }
}