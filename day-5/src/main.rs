extern crate utilities;

fn main() {
    let stacks = vec![
        vec!['D', 'B', 'J', 'V'],
        vec!['P', 'V', 'B', 'W', 'R', 'D', 'F'],
        vec!['R', 'G', 'F', 'L', 'D', 'C', 'W', 'Q'],
        vec!['W', 'J', 'P', 'M', 'L', 'N', 'D', 'B'],
        vec!['H', 'N', 'B', 'P', 'C', 'S', 'Q'],
        vec!['R', 'D', 'B', 'S', 'N', 'G'],
        vec!['Z', 'B', 'P', 'M', 'Q', 'F', 'S', 'H'],
        vec!['W', 'L', 'F'],
        vec!['S', 'V', 'F', 'M', 'R']];
    
    let ins: Vec<(i32, i32, i32)> = utilities::lines_from_file("instructions.txt").unwrap()
     .iter()
     .map(|x| parse_instruction(&x))
     .collect();

    // Part one
    let mut p1_stacks = stacks.clone();
    for (number, from, to) in &ins {
        for _ in 0..*number {
            let v = p1_stacks[*from as usize - 1].pop().unwrap();
            p1_stacks[*to as usize - 1].push(v);
        }
    }
    let p1 = p1_stacks.iter()
     .map(|x| x.last().copied().unwrap())
     .collect::<Vec<char>>();
    println!("Part one: {:?}", p1);

    // Part two
    let mut p2_stacks = stacks.clone();
    for (number, from, to) in ins {
        let mut v = Vec::new();        
        for _ in 0..number {
            v.push(p2_stacks[from as usize - 1].pop().unwrap());            
        }
        for _ in 0..number {
            p2_stacks[to as usize - 1].push(v.pop().unwrap());
        }

    }
    let p2 = p2_stacks.iter()
     .map(|x| x.last().copied().unwrap())
     .collect::<Vec<char>>();
    println!("Part two: {:?}", p2);

}


fn parse_instruction(s: &str) -> (i32, i32, i32) {
    let v = s.split(" ")
     .collect::<Vec<&str>>();
    (v[1].parse::<i32>().unwrap(), v[3].parse::<i32>().unwrap(), v[5].parse::<i32>().unwrap())
}


#[test]
fn test_parse_instruction() {
    assert_eq!(parse_instruction("move 1 from 2 to 1"), (1, 2, 1));
}