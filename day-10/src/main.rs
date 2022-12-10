extern crate utilities;
pub mod cpu;

fn main() {
    let instructions: Vec<cpu::Op> = utilities::lines_from_file("input.txt").unwrap()
     .iter()
     .map(|x| parse_op(x))
     .collect();

    let crt = cpu::CRT::new(40, 6);
    let mut cpu = cpu::CPU::new(crt);
    cpu.set_breakpoints(vec![20, 60, 100, 140, 180, 220]);
    cpu.run_program(&instructions);
    let trace = cpu.get_trace_log();
    println!("Part one: {}", trace.iter().sum::<i32>());

}

// Parse string into CPU instruction
fn parse_op(s: &str) -> cpu::Op {
    let mut i = s.split(" ");
    match i.next().unwrap() {
        "addx" => cpu::Op::AddX(i.next().unwrap().parse::<i32>().unwrap()),
        _      => cpu::Op::NoOp
    }
}

#[test]
fn test_parse_op() {
    assert_eq!(parse_op("addx -15"), cpu::Op::AddX(-15));
    assert_eq!(parse_op("noop"), cpu::Op::NoOp);
    assert_eq!(parse_op("addx 1"), cpu::Op::AddX(1));
}